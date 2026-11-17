use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A base constructor is invoked in a constructor's modifier list without
/// parentheses, eg. `constructor() B {}`. The call needs them even when it
/// passes no arguments, eg. `constructor() B() {}`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BaseConstructorCallWithoutParentheses;

impl DiagnosticExtensions for BaseConstructorCallWithoutParentheses {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/base-constructor-call-without-parentheses"
    }

    fn message(&self) -> String {
        "Modifier-style base constructor call without parentheses.".to_owned()
    }
}
