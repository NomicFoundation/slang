use std::collections::BTreeMap;

use anyhow::Result;
use semver::Version;
use slang_solidity_v2::compilation::{CompilationBuilder, CompilationBuilderConfig};
use slang_solidity_v2_common::versions::LanguageVersion;
use solidity_v2_testing_utils::reporting::diagnostic;

use crate::diagnostics_output::targets::TestTarget;
use crate::utils::path_resolver;

pub(crate) struct SlangTarget;

impl TestTarget for SlangTarget {
    fn name(&self) -> &'static str {
        "slang"
    }

    fn get_errors(
        &self,
        files: &BTreeMap<String, String>,
        version: &Version,
    ) -> Result<Vec<String>> {
        let language_version = LanguageVersion::try_from(version.clone())
            .expect("filtered to v2-supported versions above");

        let config = TestConfig {
            files: files.clone(),
        };
        let mut builder = CompilationBuilder::create(language_version, config);

        for file in files.keys() {
            builder.add_file(file.clone());
        }

        let compilation = builder.build();
        let diagnostics = compilation.diagnostics();

        let mut errors = Vec::new();
        for diagnostic in diagnostics {
            let file_id = diagnostic.file_id();
            let source = files.get(file_id).cloned().unwrap_or_default();
            errors.push(diagnostic::render(diagnostic, file_id, &source, false));
        }

        Ok(errors)
    }
}

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
