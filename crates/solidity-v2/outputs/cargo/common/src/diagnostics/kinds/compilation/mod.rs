mod missing_file;
mod unresolved_import;

pub use missing_file::MissingFile;
pub use unresolved_import::UnresolvedImport;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Group of diagnostics produced by the compilation pipeline — i.e. failures
/// that involve the file graph (reading files, resolving imports).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompilationDiagnosticKind {
    /// The compilation pipeline could not resolve an `import` directive.
    UnresolvedImport(UnresolvedImport),
    /// The configured `read_file` callback could not provide a file's source.
    MissingFile(MissingFile),
}

impl DiagnosticExtensions for CompilationDiagnosticKind {
    fn severity(&self) -> DiagnosticSeverity {
        match self {
            Self::UnresolvedImport(d) => d.severity(),
            Self::MissingFile(d) => d.severity(),
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::UnresolvedImport(d) => d.code(),
            Self::MissingFile(d) => d.code(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::UnresolvedImport(d) => d.message(),
            Self::MissingFile(d) => d.message(),
        }
    }
}
