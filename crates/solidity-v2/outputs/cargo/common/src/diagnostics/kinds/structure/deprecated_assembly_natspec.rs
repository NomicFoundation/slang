use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an assembly statement is marked as memory safe with
/// the `@solidity memory-safe-assembly` `NatSpec` comment, which the
/// `("memory-safe")` flag replaces.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DeprecatedAssemblyNatSpec;

impl DiagnosticExtensions for DeprecatedAssemblyNatSpec {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Warning
    }

    fn code(&self) -> &'static str {
        "structure/deprecated-assembly-natspec"
    }

    fn message(&self) -> String {
        "Marking an assembly block as memory safe with a '@solidity memory-safe-assembly' NatSpec comment is deprecated. Use the 'memory-safe' flag instead: 'assembly (\"memory-safe\") { ... }'.".to_string()
    }
}
