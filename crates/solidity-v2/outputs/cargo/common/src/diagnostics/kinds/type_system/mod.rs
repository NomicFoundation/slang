mod invalid_base;

pub use invalid_base::InvalidBase;
use serde::Serialize;

use crate::diagnostics::kinds::utils::define_diagnostic_kind;
use crate::diagnostics::kinds::DiagnosticKind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::TypeSystem;

    /// Group of diagnostics about the type system — type mismatches, invalid
    /// conversions, operator type errors, function-call type mismatches, and ABI
    /// encoding constraints that are properties of a type.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum TypeSystemDiagnosticKind {
        /// A base in an inheritance list is not a contract or interface.
        InvalidBase(InvalidBase),
    }
}
