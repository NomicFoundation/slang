mod operator_not_applicable_to_types;

pub use operator_not_applicable_to_types::OperatorNotApplicableToTypes;
use serde::Serialize;

use crate::diagnostics::kinds::utils::define_diagnostic_kind;
use crate::diagnostics::kinds::DiagnosticKind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::TypeSystem;

    /// Group of diagnostics produced while checking type-system rules in the
    /// semantic analysis passes.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum TypeSystemDiagnosticKind {
        /// A comparison operator was applied to operand types that don't
        /// support the operator.
        OperatorNotApplicableToTypes(OperatorNotApplicableToTypes),
    }
}
