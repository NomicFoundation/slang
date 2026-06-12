use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2::compilation::{CompilationBuilder, CompilationBuilderConfig};
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::versions::LanguageVersion;

use super::report::binder_report;
use super::report_data::ReportData;
use crate::utils::multi_part_file::split_multi_file;
use crate::utils::path_resolver;

struct TestConfig {
    files: SortedMap<String, String>,
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
        .join("binder_output")
        .join(group_name)
        .join(test_name);
    let mut fs = CodegenFileSystem::default();

    let input_path = test_dir.join("input.sol");
    let contents = input_path.read_to_string()?;

    let multi_part = split_multi_file(&contents);

    let files: SortedMap<String, String> = multi_part
        .parts
        .iter()
        .map(|part| (part.name.to_string(), part.contents.to_string()))
        .collect();

    let mut last_report = None;

    for &version in LanguageVersion::ALL {
        let config = TestConfig {
            files: files.clone(),
        };
        let mut builder = CompilationBuilder::create(version, config);

        // While `builder.add_file()` recursively adds dependencies, so adding
        // the root file would be enough, we don't want to depend on the
        // ordering of the parts in `input.sol`. Calling `add_file()` on files
        // already added is idempotent, so to be sure we add all parts.
        for file in files.keys() {
            builder.add_file(file.clone());
        }

        let compilation = builder.build();
        let report_data = ReportData::prepare(&compilation, &files);

        let status = if report_data.all_resolved() {
            "success"
        } else {
            "failure"
        };

        let report = binder_report(&report_data)?;

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
