use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A diagnostic that is emitted when `pragma experimental ABIEncoderV2` or
/// `pragma abicoder v2` is specified more than once in a Solidity source file.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DuplicateAbicoderSpecifier;

impl DiagnosticExtensions for DuplicateAbicoderSpecifier {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/duplicate-abicoder-specifier"
    }

    fn message(&self) -> String {
        "The Abicoder version has already been specified in this file.".to_string()
    }
}
