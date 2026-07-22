use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A `using {...} for` directive attached a function that is neither a
/// file-level function nor a library function.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NonFreeOrLibraryFunctionInUsingDirective;

impl DiagnosticExtensions for NonFreeOrLibraryFunctionInUsingDirective {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/non-free-or-library-function-in-using-directive"
    }

    fn message(&self) -> String {
        "Only file-level functions and library functions can be attached to a type in a \"using\" directive.".to_owned()
    }
}
