//! Tests what the compilation pipeline reports when the host declines to
//! resolve an import — the `UnresolvedImport` path, which the snapshot
//! harnesses never reach since they resolve the way `solc` does.

use slang_solidity_v2_common::evm_targets::EvmTarget;

use crate::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit, FileId};
use crate::diagnostics::kinds::compilation::{MissingFile, UnresolvedImport};
use crate::diagnostics::{DiagnosticExtensions, DiagnosticSeverity};
use crate::utils::LanguageVersion;

/// Serves one file and declines every import out of it.
struct DecliningHost {
    name: &'static str,
    contents: &'static str,
}

impl CompilationBuilderConfig for DecliningHost {
    fn read_file(&mut self, file_id: &FileId) -> Result<String, MissingFile> {
        if file_id.as_str() == self.name {
            Ok(self.contents.to_owned())
        } else {
            Err(MissingFile {
                reason: format!("no file '{file_id}'"),
            })
        }
    }

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
    let mut builder = CompilationBuilder::create(
        LanguageVersion::LATEST,
        EvmTarget::LATEST,
        DecliningHost { name, contents },
    );
    builder.add_file(name.into());

    builder.build()
}

#[test]
fn declined_import_is_reported_as_unresolved() {
    let contents = "import {Foo} from \"@scope/pkg/foo.sol\";\ncontract Main {}\n";
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

    // The host's reason is carried through verbatim.
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
        "import \"a.sol\";\nimport \"b.sol\";\nimport {C} from \"c.sol\";\ncontract Main {}\n",
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
