mod missing_file;
mod missing_imported_file;
mod unresolved_import;

pub use missing_file::MissingFile;
pub use missing_imported_file::MissingImportedFile;
use serde::Serialize;
pub use unresolved_import::UnresolvedImport;

use crate::diagnostics::kinds::utils::define_diagnostic_kind;
use crate::diagnostics::kinds::DiagnosticKind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::Compilation;

    /// Group of diagnostics produced by the compilation pipeline — i.e. failures
    /// that involve the file graph (reading files, resolving imports).
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum CompilationDiagnosticKind {
        /// The compilation pipeline could not resolve an `import` directive.
        UnresolvedImport(UnresolvedImport),
        /// The configured `read_file` callback could not provide a file's source.
        MissingFile(MissingFile),
        /// An `import` directive resolved to a file that could not be read.
        MissingImportedFile(MissingImportedFile),
    }
}
