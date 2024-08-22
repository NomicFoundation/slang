use napi_derive::napi;

use crate::napi_interface::cst::TextRange;

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
    #[napi(ts_return_type = "cst.TextRange")]
    pub fn text_range(&self) -> TextRange {
        self.0.text_range().into()
    }

    /// The primary message associated with this diagnostic.
    #[napi]
    pub fn message(&self) -> String {
        self.0.message()
    }
}

/// Exposes the [`Diagnostic`](crate::diagnostic::Diagnostic) methods implemented for a given type
/// as regular N-API functions, satisfying the custom `DiagnosticInterface` interface on the N-API side.
// NOTE(#987): This is required because we can't directly expose the trait using `#[napi]` or expose
// an interface that has only methods defined.
// To not require explicit conversions, we define the interface ourselves and expose the relevant methods to satisfy it.
#[macro_export]
macro_rules! expose_diagnostic_trait_interface {
    ($namespace:literal, $diagnostic:ty) => {
        #[napi(namespace = $namespace)]
        impl $diagnostic {
            #[napi(ts_return_type = "diagnostic.Severity", catch_unwind)]
            pub fn severity(&self) -> $crate::napi_interface::diagnostic::Severity {
                $crate::diagnostic::Diagnostic::severity(&self.0).into()
            }

            #[napi(ts_return_type = "cst.TextRange", catch_unwind)]
            pub fn text_range(&self) -> $crate::napi_interface::cst::TextRange {
                $crate::diagnostic::Diagnostic::text_range(&self.0).into()
            }

            #[napi]
            pub fn message(&self) -> String {
                $crate::diagnostic::Diagnostic::message(&self.0)
            }
        }
    };
}
