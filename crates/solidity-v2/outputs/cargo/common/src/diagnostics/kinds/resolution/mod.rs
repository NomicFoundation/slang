mod built_in_redeclaration;
mod external_declaration_shadowing;
mod identifier_not_found;
mod identifier_redeclaration;
mod incompatible_built_in_target;
mod incompatible_built_in_version;

pub use built_in_redeclaration::BuiltInRedeclaration;
pub use external_declaration_shadowing::ExternalDeclarationShadowing;
pub use identifier_not_found::IdentifierNotFound;
pub use identifier_redeclaration::IdentifierRedeclaration;
pub use incompatible_built_in_target::IncompatibleBuiltInTarget;
pub use incompatible_built_in_version::IncompatibleBuiltInVersion;
use serde::Serialize;

use crate::diagnostics::kinds::utils::define_diagnostic_kind;
use crate::diagnostics::kinds::DiagnosticKind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::Resolution;

    /// Group of diagnostics for undeclared identifiers, duplicate
    /// definitions, import failures, shadowing, ambiguous references,
    /// scope errors, and incompatible built-ins.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum ResolutionDiagnosticKind {
        /// An identifier could not be resolved.
        IdentifierNotFound(IdentifierNotFound),
        /// An identifier was declared more than once in the same scope.
        IdentifierRedeclaration(IdentifierRedeclaration),
        /// A Yul declaration reused the reserved name of a Yul built-in.
        BuiltInRedeclaration(BuiltInRedeclaration),
        /// A Yul variable declaration shadows a declaration (Solidity or
        /// built-in) visible from outside the assembly block.
        ExternalDeclarationShadowing(ExternalDeclarationShadowing),

        /// A built-in is not compatible with the currently selected language version.
        IncompatibleBuiltInVersion(IncompatibleBuiltInVersion),
        /// A built-in is not compatible with the currently selected EVM target.
        IncompatibleBuiltInTarget(IncompatibleBuiltInTarget),
    }
}
