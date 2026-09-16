use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// The kind of declaration an assembly reference names, when it is not a
/// variable or a library.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum UnsupportedReferenceKind {
    Contract,
    Enum,
    Error,
    Event,
    Function,
    Import,
    Interface,
    Modifier,
    Struct,
    UserDefinedValueType,
}

/// Diagnostic emitted at an assembly reference that reads a declaration that
/// is neither a variable nor a library.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct YulUnsupportedReference {
    /// The kind of declaration the reference names.
    pub kind: UnsupportedReferenceKind,
}

impl DiagnosticExtensions for YulUnsupportedReference {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "semantic/yul-unsupported-reference"
    }

    fn message(&self) -> String {
        let named = match self.kind {
            UnsupportedReferenceKind::Contract => "a contract type",
            UnsupportedReferenceKind::Enum => "an enum type",
            UnsupportedReferenceKind::Error => "an error",
            UnsupportedReferenceKind::Event => "an event",
            UnsupportedReferenceKind::Function => "a function",
            UnsupportedReferenceKind::Import => "an import",
            UnsupportedReferenceKind::Interface => "an interface type",
            UnsupportedReferenceKind::Modifier => "a modifier",
            UnsupportedReferenceKind::Struct => "a struct type",
            UnsupportedReferenceKind::UserDefinedValueType => "a user defined value type",
        };
        format!(
            "This is {named}. Only variables and libraries can be referenced in inline assembly."
        )
    }
}
