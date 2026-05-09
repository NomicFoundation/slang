mod extra_terminal;
mod unexpected_eof;
mod unexpected_terminal;
mod unsupported_syntax;

pub use extra_terminal::ExtraTerminal;
use serde::Serialize;
use slang_solidity_v2_common::define_diagnostic_kind;
pub use unexpected_eof::UnexpectedEof;
pub use unexpected_terminal::UnexpectedTerminal;
pub use unsupported_syntax::UnsupportedSyntax;

define_diagnostic_kind! {
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
    }
}
