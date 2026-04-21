mod extra_terminal;
mod unexpected_eof;
mod unexpected_terminal;
mod unsupported_syntax;

pub use extra_terminal::ExtraTerminal;
pub use unexpected_eof::UnexpectedEof;
pub use unexpected_terminal::UnexpectedTerminal;
pub use unsupported_syntax::UnsupportedSyntax;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Group of diagnostics produced while parsing source text.
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl DiagnosticExtensions for SyntaxDiagnosticKind {
    fn severity(&self) -> DiagnosticSeverity {
        match self {
            Self::UnexpectedEof(d) => d.severity(),
            Self::UnexpectedTerminal(d) => d.severity(),
            Self::ExtraTerminal(d) => d.severity(),
            Self::UnsupportedSyntax(d) => d.severity(),
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::UnexpectedEof(d) => d.code(),
            Self::UnexpectedTerminal(d) => d.code(),
            Self::ExtraTerminal(d) => d.code(),
            Self::UnsupportedSyntax(d) => d.code(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::UnexpectedEof(d) => d.message(),
            Self::UnexpectedTerminal(d) => d.message(),
            Self::ExtraTerminal(d) => d.message(),
            Self::UnsupportedSyntax(d) => d.message(),
        }
    }
}
