mod ambiguous_reference;
mod ambiguous_yul_reference;
mod built_in_redeclaration;
mod duplicate_event_definition;
mod duplicate_function_definition;
mod external_declaration_shadowing;
mod identifier_not_found;
mod identifier_not_function_or_not_unique;
mod identifier_not_library_name;
mod identifier_redeclaration;
mod imported_declaration_not_found;
mod incompatible_built_in_target;
mod incompatible_built_in_version;
mod non_free_or_library_function_in_using_directive;

pub use ambiguous_reference::AmbiguousReference;
pub use ambiguous_yul_reference::AmbiguousYulReference;
pub use built_in_redeclaration::BuiltInRedeclaration;
pub use duplicate_event_definition::DuplicateEventDefinition;
pub use duplicate_function_definition::DuplicateFunctionDefinition;
pub use external_declaration_shadowing::ExternalDeclarationShadowing;
pub use identifier_not_found::IdentifierNotFound;
pub use identifier_not_function_or_not_unique::IdentifierNotFunctionOrNotUnique;
pub use identifier_not_library_name::IdentifierNotLibraryName;
pub use identifier_redeclaration::IdentifierRedeclaration;
pub use imported_declaration_not_found::ImportedDeclarationNotFound;
pub use incompatible_built_in_target::IncompatibleBuiltInTarget;
pub use incompatible_built_in_version::IncompatibleBuiltInVersion;
pub use non_free_or_library_function_in_using_directive::NonFreeOrLibraryFunctionInUsingDirective;
use serde::Serialize;

use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::kinds::utils::define_diagnostic_kind;

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
        /// Two functions visible under the same name have parameter lists that
        /// an external call cannot tell apart.
        DuplicateFunctionDefinition(DuplicateFunctionDefinition),
        /// Two events visible under the same name have parameter lists that an
        /// external call cannot tell apart.
        DuplicateEventDefinition(DuplicateEventDefinition),
        /// A Yul declaration reused the reserved name of a Yul built-in.
        BuiltInRedeclaration(BuiltInRedeclaration),
        /// A Yul variable declaration shadows a declaration (Solidity or
        /// built-in) visible from outside the assembly block.
        ExternalDeclarationShadowing(ExternalDeclarationShadowing),
        /// A symbol in an import deconstruction is not declared in the
        /// imported file.
        ImportedDeclarationNotFound(ImportedDeclarationNotFound),
        /// A reference matches more than one declaration, and neither the call
        /// arguments nor the context narrow it down to a single one.
        AmbiguousReference(AmbiguousReference),
        /// A Yul identifier matches more than one declaration, which Yul cannot
        /// disambiguate.
        AmbiguousYulReference(AmbiguousYulReference),

        /// A symbol in a `using {...} for` directive did not resolve to a
        /// unique function.
        IdentifierNotFunctionOrNotUnique(IdentifierNotFunctionOrNotUnique),
        /// A `using {...} for` directive attached a function that is neither
        /// file-level nor a library function.
        NonFreeOrLibraryFunctionInUsingDirective(NonFreeOrLibraryFunctionInUsingDirective),
        /// The library name in a `using ... for` directive did not resolve to
        /// a unique library.
        IdentifierNotLibraryName(IdentifierNotLibraryName),

        /// A built-in is not compatible with the currently selected language version.
        IncompatibleBuiltInVersion(IncompatibleBuiltInVersion),
        /// A built-in is not compatible with the currently selected EVM target.
        IncompatibleBuiltInTarget(IncompatibleBuiltInTarget),
    }
}
