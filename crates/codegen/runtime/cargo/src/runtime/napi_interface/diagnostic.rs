use napi_derive::napi;

use crate::napi_interface::text_index::TextRange;

/// Severity of the compiler diagnostic.
///
/// Explicitly compatible with the LSP protocol.
#[napi(namespace = "diagnostic")]
pub enum Severity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

impl From<crate::diagnostic::Severity> for Severity {
    fn from(value: crate::diagnostic::Severity) -> Severity {
        match value {
            crate::diagnostic::Severity::Error => Self::Error,
            crate::diagnostic::Severity::Warning => Self::Warning,
            crate::diagnostic::Severity::Information => Self::Information,
            crate::diagnostic::Severity::Hint => Self::Hint,
        }
    }
}

/// A compiler diagnostic that can be rendered to a user.
#[napi(namespace = "diagnostic")]
pub struct Diagnostic(pub(crate) Box<dyn crate::diagnostic::Diagnostic>);

#[napi(namespace = "diagnostic")]
impl Diagnostic {
    /// The severity of this diagnostic.
    #[napi]
    pub fn severity(&self) -> Severity {
        self.0.severity().into()
    }

    /// The character range of the source that this diagnostic applies to.
    ///
    /// Note that this is not tracking columns, so it is not compatible with LSP's
    /// [`Position`](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#position)
    /// at the moment.
    #[napi(ts_return_type = "text_index.TextRange")]
    pub fn text_range(&self) -> TextRange {
        self.0.range().into()
    }

    /// The primary message associated with this diagnostic.
    #[napi]
    pub fn message(&self) -> String {
        self.0.message()
    }

    /// The code (i.e. `S0123`), if any, associated with this diagnostic.
    #[napi]
    pub fn code(&self) -> String {
        self.0.code().unwrap_or_default().into_owned()
    }
}
