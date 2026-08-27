#[path = "incompatible_syntax_version.generated.rs"]
mod incompatible_syntax_version;
mod missing_version_pragma;

use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::structured_cst::nodes::SourceUnit;

pub fn validate_cst(
    source_unit: &SourceUnit,
    file_id: &FileId,
    language_version: LanguageVersion,
    diagnostics: &mut DiagnosticCollection,
) {
    // This validation must run on the CST, as some nodes are rewritten into higher-level representations when we build the IR.
    incompatible_syntax_version::validate(source_unit, language_version, file_id, diagnostics);

    // This validation must also run on the CST, given that we don't have parser error recovery yet.
    // Once the parser can recover/return top-level pragmas in the precence of syntax errors, we can consider moving it to the IR.
    missing_version_pragma::validate(source_unit, file_id, diagnostics);
}
