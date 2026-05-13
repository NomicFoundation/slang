use std::ops::Range;

use slang_solidity_v2_common::diagnostics::{Diagnostic, DiagnosticExtensions, DiagnosticSeverity};

use super::RenderDiagnostic;

impl RenderDiagnostic for Diagnostic {
    fn text_range(&self) -> Range<usize> {
        Diagnostic::text_range(self).clone()
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticExtensions::severity(self)
    }

    fn message(&self) -> String {
        DiagnosticExtensions::message(self)
    }
}
