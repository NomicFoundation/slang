use std::collections::BTreeMap;
use std::fmt::Write;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2::compilation::builder::{CompilationBuilder, CompilationBuilderConfig};
use slang_solidity_v2_common::versions::LanguageVersion;
use solidity_v2_language::SolidityDefinition;
use solidity_v2_testing_utils::reporting::diagnostic;

use crate::utils::multi_part_file::split_multi_file;
use crate::utils::path_resolver;

struct TestConfig {
    files: BTreeMap<String, String>,
}

impl CompilationBuilderConfig for TestConfig {
    fn read_file(&mut self, file_id: &str) -> Result<String, String> {
        self.files
            .get(file_id)
            .cloned()
            .ok_or_else(|| format!("file not found: {file_id}"))
    }

    fn resolve_import(
        &mut self,
        source_file_id: &str,
        import_path: &str,
    ) -> Result<String, String> {
        path_resolver::resolve_import(source_file_id, import_path)
            .ok_or_else(|| format!("unresolved import: {import_path} (from {source_file_id})"))
    }
}

pub(crate) fn run(group_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?
        .join("diagnostics_output")
        .join(group_name)
        .join(test_name);
    let mut fs = CodegenFileSystem::default();

    let input_path = test_dir.join("input.sol");
    let contents = input_path.read_to_string()?;

    let multi_part = split_multi_file(&contents);

    // Use a sorted map so file iteration order is deterministic across runs.
    let files: BTreeMap<String, String> = multi_part
        .parts
        .iter()
        .map(|part| (part.name.to_string(), part.contents.to_string()))
        .collect();

    let tested_versions: Vec<LanguageVersion> = SolidityDefinition::create()
        .collect_breaking_versions()
        .into_iter()
        .filter_map(|version| LanguageVersion::try_from(version).ok())
        .collect();

    let mut last_report = None;

    for &version in &tested_versions {
        let config = TestConfig {
            files: files.clone(),
        };
        let mut builder = CompilationBuilder::create(version, config);

        for file in files.keys() {
            builder.add_file(file.clone());
        }

        let compilation = builder.build();
        let diagnostics = compilation.diagnostics();

        let count = diagnostics.iter().count();
        let status = if count == 0 { "success" } else { "failure" };

        let mut report = String::new();
        writeln!(report, "Diagnostics: {count}")?;
        for diagnostic in diagnostics {
            let file_id = diagnostic.file_id();
            let source = files.get(file_id).cloned().unwrap_or_default();
            let rendered = diagnostic::render(diagnostic, file_id, &source, false);
            writeln!(report)?;
            writeln!(report, "{rendered}")?;
        }

        match last_report {
            Some(ref last) if last == &report => (),
            _ => {
                let snapshot_path = test_dir
                    .join("generated")
                    .join(format!("{version}-{status}.txt"));
                fs.write_file_raw(snapshot_path, &report)?;
                last_report = Some(report);
            }
        }
    }

    Ok(())
}
