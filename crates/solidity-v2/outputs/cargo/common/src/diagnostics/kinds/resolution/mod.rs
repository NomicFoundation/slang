mod identifier_not_found;
mod identifier_redeclaration;

pub use identifier_not_found::IdentifierNotFound;
pub use identifier_redeclaration::IdentifierRedeclaration;
use serde::Serialize;

use crate::diagnostics::kinds::utils::define_diagnostic_kind;
use crate::diagnostics::kinds::DiagnosticKind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::Resolution;

    /// Group of diagnostics for undeclared identifiers, duplicate
    /// definitions, import failures, shadowing, ambiguous references
    /// and scope errors.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum ResolutionDiagnosticKind {
        /// An identifier could not be resolved.
        IdentifierNotFound(IdentifierNotFound),
        /// An identifier was declared more than once in the same scope.
        IdentifierRedeclaration(IdentifierRedeclaration),
    }
}
