use slang_solidity_v2_common::evm_targets::EvmTarget;

use crate::compilation::{CompilationUnit, Configuration, FileId, ImportResolver};
use crate::diagnostics::kinds::compilation::UnresolvedImport;
use crate::utils::LanguageVersion;

/// Resolves every import path to a file of the same name.
struct TestImportResolver;

impl ImportResolver for TestImportResolver {
    fn resolve_import(
        &mut self,
        _source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        Ok(import_path.into())
    }
}

fn compile(sources: impl IntoIterator<Item = (FileId, String)>) -> CompilationUnit {
    CompilationUnit::create(Configuration {
        language_version: LanguageVersion::LATEST,
        evm_target: EvmTarget::LATEST,
        sources,
        resolver: TestImportResolver,
    })
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
fn compiles_every_source_it_is_given() {
    let unit = compile([
        ("main.sol".into(), contract("Main", &["lib.sol"])),
        ("lib.sol".into(), contract("Lib", &[])),
        // Not imported by anything, but still part of the compilation.
        ("extra.sol".into(), contract("Extra", &[])),
    ]);

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
fn the_last_contents_given_for_a_file_id_win() {
    let unit = compile([
        ("main.sol".into(), contract("Stale", &[])),
        ("main.sol".into(), contract("Fresh", &[])),
    ]);

    assert!(unit.diagnostics().is_empty(), "{:#?}", unit.diagnostics());
    assert_eq!(unit.files().count(), 1);

    let contract_names: Vec<String> = unit
        .all_contracts()
        .map(|contract| contract.name().name().to_owned())
        .collect();
    assert_eq!(contract_names, ["Fresh"]);
}
