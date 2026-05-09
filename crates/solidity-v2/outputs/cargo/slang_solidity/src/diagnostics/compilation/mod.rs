mod missing_file;
mod unresolved_import;

pub use missing_file::MissingFile;
use serde::Serialize;
use slang_solidity_v2_common::define_diagnostic_kind;
pub use unresolved_import::UnresolvedImport;

use super::DiagnosticKind;

define_diagnostic_kind! {
    /// Group of diagnostics produced by the compilation pipeline — i.e. failures
    /// that involve the file graph (reading files, resolving imports).
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum CompilationDiagnosticKind {
        /// The compilation pipeline could not resolve an `import` directive.
        UnresolvedImport(UnresolvedImport),
        /// The configured `read_file` callback could not provide a file's source.
        MissingFile(MissingFile),
    }
}

// Required to be able to push these diagnostics directly into the
// `DiagnosticCollection<DiagnosticKind>`
impl From<MissingFile> for DiagnosticKind {
    fn from(d: MissingFile) -> Self {
        Self::Compilation(d.into())
    }
}

impl From<UnresolvedImport> for DiagnosticKind {
    fn from(d: UnresolvedImport) -> Self {
        Self::Compilation(d.into())
    }
}
