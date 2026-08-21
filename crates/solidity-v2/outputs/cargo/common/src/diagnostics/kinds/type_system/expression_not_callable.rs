use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the callee of a call cannot be called: a value of a
/// non-function type (eg. `1(2)`, or a mapping, which is indexed instead), a
/// declaration that names no callable (eg. a modifier or an import alias), a
/// built-in namespace such as `msg`, or `this`/`super`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExpressionNotCallable;

impl DiagnosticExtensions for ExpressionNotCallable {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/expression-not-callable"
    }

    fn message(&self) -> String {
        "This expression is not callable.".to_owned()
    }
}
