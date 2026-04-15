use std::collections::HashMap;
use std::convert::Infallible;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2::compilation::builder::{
    CompilationBuilder, CompilationBuilderConfig, CompilationBuilderError,
};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::ParserError;
use solidity_v2_language::SolidityDefinition;

use super::report::binder_report;
use super::report_data::ReportData;
use crate::utils::multi_part_file::split_multi_file;
use crate::utils::path_resolver;

struct TestConfig {
    files: HashMap<String, String>,
}

impl CompilationBuilderConfig for TestConfig {
    type Error = Infallible;

    fn read_file(&mut self, file_id: &str) -> Result<Option<String>, Self::Error> {
        Ok(self.files.get(file_id).cloned())
    }

    fn resolve_import(
        &mut self,
        source_file_id: &str,
        import_path: &str,
    ) -> Result<Option<String>, Self::Error> {
        Ok(path_resolver::resolve_import(source_file_id, import_path))
    }
}

pub(crate) fn run(group_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?
        .join("semantic_output")
        .join(group_name)
        .join(test_name);
    let mut fs = CodegenFileSystem::default();

    let input_path = test_dir.join("input.sol");
    let contents = input_path.read_to_string()?;

    let multi_part = split_multi_file(&contents);

    let files: HashMap<String, String> = multi_part
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

        let mut parse_errors: Vec<(String, ParserError)> = Vec::new();
        // While `builder.add_file()` recursively adds dependencies, so adding
        // the root file would be enough, we don't want to depend on the
        // ordering of the parts in `input.sol`. Calling `add_file()` on files
        // already added is idempotent, so to be sure we add all parts.
        for file in files.keys() {
            match builder.add_file(file) {
                Ok(()) => {}
                Err(CompilationBuilderError::ParserError(errors)) => {
                    for error in errors {
                        parse_errors.push((file.clone(), error));
                    }
                }
                Err(CompilationBuilderError::UserError(infallible)) => match infallible {},
            }
        }

        let compilation = builder.build();
        let report_data = ReportData::prepare(&compilation, &files, parse_errors);

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
