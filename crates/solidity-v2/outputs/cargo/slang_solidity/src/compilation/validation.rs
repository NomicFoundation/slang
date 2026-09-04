use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::syntax::MissingVersionPragma;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_cst::structured_cst::nodes::{Pragma, SourceUnit, SourceUnitMember};

pub fn validate_cst(
    source_unit: &SourceUnit,
    file_id: &FileId,
    diagnostics: &mut DiagnosticCollection,
) {
    missing_version_pragma(source_unit, file_id, diagnostics);
}

fn missing_version_pragma(
    source_unit: &SourceUnit,
    file_id: &FileId,
    diagnostics: &mut DiagnosticCollection,
) {
    let has_version_pragma = source_unit.members.elements.iter().any(|member| {
        matches!(
            member,
            SourceUnitMember::PragmaDirective(directive)
                if matches!(directive.pragma, Pragma::VersionPragma(_))
        )
    });

    if !has_version_pragma {
        diagnostics.push(file_id.to_owned(), 0..0, MissingVersionPragma);
    }
}
