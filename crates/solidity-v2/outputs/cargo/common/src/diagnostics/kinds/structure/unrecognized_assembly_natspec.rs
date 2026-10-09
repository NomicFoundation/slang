use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an assembly statement is documented by a `NatSpec`
/// comment other than the one marking it as memory safe: a single
/// `/// @solidity memory-safe-assembly` line.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UnrecognizedAssemblyNatSpec;

impl DiagnosticExtensions for UnrecognizedAssemblyNatSpec {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Warning
    }

    fn code(&self) -> &'static str {
        "structure/unrecognized-assembly-natspec"
    }

    fn message(&self) -> String {
        "The only NatSpec comment allowed on an assembly block is '/// @solidity memory-safe-assembly', on a single line, which marks it as memory safe.".to_string()
    }
}
