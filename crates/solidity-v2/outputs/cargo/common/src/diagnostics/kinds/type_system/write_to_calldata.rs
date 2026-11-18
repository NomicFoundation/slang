use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// What is written to, where it lives in calldata.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum CalldataWriteKind {
    /// An element of a calldata array.
    ArrayElement,
    /// A member of a calldata struct.
    StructMember,
}

/// Diagnostic emitted when an element of a calldata array or a member of a
/// calldata struct is written to (assigned to, deleted, incremented or
/// decremented). Calldata is the read-only input of an external call.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WriteToCalldata {
    /// What is written to.
    pub kind: CalldataWriteKind,
}

impl DiagnosticExtensions for WriteToCalldata {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/write-to-calldata"
    }

    fn message(&self) -> String {
        match self.kind {
            CalldataWriteKind::ArrayElement => "Calldata arrays are read-only.",
            CalldataWriteKind::StructMember => "Calldata structs are read-only.",
        }
        .to_owned()
    }
}
