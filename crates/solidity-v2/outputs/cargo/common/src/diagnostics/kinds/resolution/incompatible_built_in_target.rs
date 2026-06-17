use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::evm_targets::EvmTargetSpecifier;

/// A built-in is not compatible with the currently selected EVM target.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IncompatibleBuiltInTarget {
    /// The range of EVM targets in which this built-in is compatible.
    pub compatible_in: EvmTargetSpecifier,
}

impl DiagnosticExtensions for IncompatibleBuiltInTarget {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/incompatible-built-in-target"
    }

    fn message(&self) -> String {
        match &self.compatible_in {
            EvmTargetSpecifier::From { from } => {
                format!("This built-in was introduced in EVM target '{from}'.")
            }
            EvmTargetSpecifier::Till { till } => {
                format!("This built-in was deprecated in EVM target '{till}'.")
            }
            EvmTargetSpecifier::Range { from, till } => {
                format!(
                    "This built-in was introduced in EVM target '{from}', and deprecated in EVM target '{till}'."
                )
            }
        }
    }
}
