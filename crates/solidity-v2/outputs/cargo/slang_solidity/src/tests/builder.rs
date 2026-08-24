use slang_solidity_v2_common::evm_targets::EvmTarget;

use crate::compilation::{CompilationBuilder, CompilationBuilderConfig, FileId};
use crate::diagnostics::kinds::compilation::UnresolvedImport;
use crate::utils::LanguageVersion;

/// Resolves every import path to a file of the same name.
struct TestConfig;

impl CompilationBuilderConfig for TestConfig {
    fn resolve_import(
        &mut self,
        _source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        Ok(import_path.into())
    }
}

fn builder() -> CompilationBuilder<TestConfig> {
    CompilationBuilder::create(LanguageVersion::LATEST, EvmTarget::LATEST, TestConfig)
}

fn contract(name: &str, imports: &[&str]) -> String {
    use std::fmt::Write;

    let imports = imports.iter().fold(String::new(), |mut text, path| {
        writeln!(text, "import \"{path}\";").unwrap();
        text
    });

    format!("pragma solidity ^0.8.0;\n{imports}\ncontract {name} {{}}\n")
}

#[test]
fn builds_every_file_that_was_added() {
    let mut builder = builder();

    builder.add_files([
        ("main.sol".into(), contract("Main", &["lib.sol"])),
        ("lib.sol".into(), contract("Lib", &[])),
    ]);
    // Not imported by anything, but still part of the compilation.
    builder.add_file("extra.sol".into(), contract("Extra", &[]));

    let unit = builder.build();

    assert!(unit.diagnostics().is_empty(), "{:#?}", unit.diagnostics());

    // `files()` yields the files sorted by ID, but that order is not part of
    // the API, so sort here and assert on the set of files instead.
    let mut file_ids: Vec<String> = unit
        .files()
        .map(|file| file.id().as_str().to_owned())
        .collect();
    file_ids.sort();
    assert_eq!(file_ids, ["extra.sol", "lib.sol", "main.sol"]);
}

#[test]
fn adding_a_file_twice_replaces_its_contents() {
    let mut builder = builder();

    builder.add_file("main.sol".into(), contract("Stale", &[]));
    builder.add_file("main.sol".into(), contract("Fresh", &[]));

    let unit = builder.build();

    assert!(unit.diagnostics().is_empty(), "{:#?}", unit.diagnostics());
    assert_eq!(unit.files().count(), 1);

    let contract_names: Vec<String> = unit
        .all_contracts()
        .map(|contract| contract.name().name().to_owned())
        .collect();
    assert_eq!(contract_names, ["Fresh"]);
}
