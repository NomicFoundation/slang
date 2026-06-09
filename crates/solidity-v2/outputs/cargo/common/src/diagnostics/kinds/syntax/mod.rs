mod expected_array_length_expression;
mod extra_terminal;
mod multiple_mutability_specifiers;
mod multiple_override_specifiers;
mod multiple_virtual_specifiers;
mod unexpected_eof;
mod unexpected_terminal;
mod unsupported_syntax;

pub use expected_array_length_expression::ExpectedArrayLengthExpression;
pub use extra_terminal::ExtraTerminal;
pub use multiple_mutability_specifiers::MultipleMutabilitySpecifiers;
pub use multiple_override_specifiers::MultipleOverrideSpecifiers;
pub use multiple_virtual_specifiers::MultipleVirtualSpecifiers;
use serde::Serialize;
pub use unexpected_eof::UnexpectedEof;
pub use unexpected_terminal::UnexpectedTerminal;
pub use unsupported_syntax::UnsupportedSyntax;

use crate::diagnostics::kinds::utils::define_diagnostic_kind;
use crate::diagnostics::kinds::DiagnosticKind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::Syntax;

    /// Group of diagnostics produced while parsing source text.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum SyntaxDiagnosticKind {
        /// The parser reached end-of-file while still expecting terminals.
        UnexpectedEof(UnexpectedEof),
        /// The parser encountered a terminal not valid at the current position.
        UnexpectedTerminal(UnexpectedTerminal),
        /// The parser encountered extra input after a complete source unit.
        ExtraTerminal(ExtraTerminal),

        /// A piece of syntax is not supported by the currently selected version.
        UnsupportedSyntax(UnsupportedSyntax),

        /// More than one mutability specifier was provided on a definition.
        MultipleMutabilitySpecifiers(MultipleMutabilitySpecifiers),

        /// More than one `virtual` specifier was provided on a definition.
        MultipleVirtualSpecifiers(MultipleVirtualSpecifiers),

        /// A range/slice index access was used where an array length
        /// expression is expected
        ExpectedArrayLengthExpression(ExpectedArrayLengthExpression),

        /// More than one `override` specifier was provided on a definition.
        MultipleOverrideSpecifiers(MultipleOverrideSpecifiers),
    }
}
