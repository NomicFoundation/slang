use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::syntax::MissingVersionPragma;
use slang_solidity_v2_common::files::FileId;

use crate::structured_cst::nodes::{Pragma, SourceUnit, SourceUnitMember};

/// Validate that the given `SourceUnit` names the compiler version it was written for.
pub fn validate(root: &SourceUnit, file_id: &FileId, diagnostics: &mut DiagnosticCollection) {
    let has_version_pragma = root.members.elements.iter().any(|member| {
        matches!(
            member,
            SourceUnitMember::PragmaDirective(directive)
                if matches!(directive.pragma, Pragma::VersionPragma(_))
        )
    });

    if !has_version_pragma {
        // There is no node to point at, so the warning attaches to the start of the file.
        diagnostics.push(file_id.to_owned(), 0..0, MissingVersionPragma);
    }
}
