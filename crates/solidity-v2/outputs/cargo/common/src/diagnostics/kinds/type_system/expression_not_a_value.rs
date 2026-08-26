use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// What an expression names, where a value was required.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum NotAValueKind {
    /// A built-in: a namespace such as `abi`, or a built-in function such as
    /// `keccak256`.
    BuiltIn,
    /// The `super` base-contract reference.
    Super,
    /// A type reference.
    TypeOrModule,
    /// A `new` contract creation that is never called.
    UncalledNew,
}

/// Diagnostic emitted when an expression is used where a value is required but
/// it names something else. A statement is not such a position: naming a
/// built-in or `super` there is inert and accepted.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExpressionNotAValue {
    /// What the expression names instead of a value.
    pub kind: NotAValueKind,
}

impl DiagnosticExtensions for ExpressionNotAValue {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/expression-not-a-value"
    }

    fn message(&self) -> String {
        let named = match self.kind {
            NotAValueKind::BuiltIn => "This built-in",
            NotAValueKind::Super => "'super'",
            NotAValueKind::TypeOrModule => "This expression denoting a type or module",
            NotAValueKind::UncalledNew => "An uncalled 'new' expression",
        };
        format!("{named} cannot be used as a value.")
    }
}
