//! Tests specifically for builder configurations that don't resolve every import

use slang_solidity_v2_common::evm_targets::EvmTarget;

use crate::compilation::{CompilationUnit, Configuration, FileId, ImportResolver};
use crate::diagnostics::kinds::compilation::UnresolvedImport;
use crate::diagnostics::{DiagnosticExtensions, DiagnosticSeverity};
use crate::utils::LanguageVersion;

/// Serves one file and declines every import out of it.
struct DecliningImportResolver;

impl ImportResolver for DecliningImportResolver {
    fn resolve_import(
        &mut self,
        _source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        Err(UnresolvedImport {
            reason: format!("no remapping covers '{import_path}'"),
        })
    }
}

fn compile(name: &'static str, contents: &'static str) -> CompilationUnit {
    CompilationUnit::create(Configuration {
        language_version: LanguageVersion::LATEST,
        evm_target: EvmTarget::LATEST,
        sources: [(name.into(), contents)],
        resolver: DecliningImportResolver,
    })
}

#[test]
fn declined_import_is_reported_as_unresolved() {
    let contents = r#"
        pragma solidity *;
        import {Foo} from "@scope/pkg/foo.sol";
        contract Main {}
    "#;
    let unit = compile("main.sol", contents);

    let diagnostics: Vec<_> = unit.diagnostics().iter().collect();
    assert_eq!(
        1,
        diagnostics.len(),
        "expected exactly the unresolved import, but found: {diagnostics:#?}"
    );

    let diagnostic = diagnostics[0];
    assert_eq!("compilation/unresolved-import", diagnostic.code());
    assert_eq!(DiagnosticSeverity::Error, diagnostic.severity());

    // The builder config's reason is carried through verbatim.
    assert_eq!(
        "no remapping covers '@scope/pkg/foo.sol'",
        diagnostic.message()
    );

    // Reported against the import, in the file that wrote it.
    assert_eq!("main.sol", diagnostic.file_id().as_str());
    assert_eq!(
        "\"@scope/pkg/foo.sol\"",
        &contents[diagnostic.text_range().clone()]
    );
}

/// Every declined import is reported, not just the first.
#[test]
fn every_declined_import_is_reported() {
    let unit = compile(
        "main.sol",
        r#"
        pragma solidity *;
        import "a.sol";
        import "b.sol";
        import {C} from "c.sol";
        contract Main {}
        "#,
    );

    let messages: Vec<_> = unit
        .diagnostics()
        .iter()
        .map(DiagnosticExtensions::message)
        .collect();

    assert_eq!(
        vec![
            "no remapping covers 'a.sol'",
            "no remapping covers 'b.sol'",
            "no remapping covers 'c.sol'",
        ],
        messages
    );
}
