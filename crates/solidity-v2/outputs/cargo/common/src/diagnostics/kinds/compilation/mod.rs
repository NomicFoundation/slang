mod missing_imported_file;
mod unresolved_import;

pub use missing_imported_file::MissingImportedFile;
use serde::Serialize;
pub use unresolved_import::UnresolvedImport;

use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::kinds::utils::define_diagnostic_kind;

define_diagnostic_kind! {
    parent_kind = DiagnosticKind::Compilation;

    /// Group of diagnostics produced by the compilation pipeline — i.e. failures
    /// that involve the file graph (resolving imports to the files provided).
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum CompilationDiagnosticKind {
        /// The compilation pipeline could not resolve an `import` directive.
        UnresolvedImport(UnresolvedImport),
        /// An `import` directive resolved to a file that is not part of the
        /// compilation.
        MissingImportedFile(MissingImportedFile),
    }
}
