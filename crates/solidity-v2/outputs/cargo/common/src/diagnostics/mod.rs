//! Unified diagnostic API used by the Slang v2 parser and compilation
//! pipeline.
//!
//! Every non-panicking failure — parse errors, syntax-version mismatches,
//! unresolvable imports, missing files — is surfaced uniformly as a
//! [`Diagnostic`] collected in a [`DiagnosticCollection`]. Each `Diagnostic`
//! carries its location (file identifier and byte range) plus a `kind` field
//! describing the specific problem using specific diagnostic types.
//!
//! Rendering concerns — severity, a stable machine-readable code, and a
//! human-readable message — live behind the [`DiagnosticExtensions`] trait,
//! which is implemented for every leaf diagnostic, every group enum, and the
//! top-level `Diagnostic` wrapper.

mod collection;
mod diagnostic;
mod extensions;
mod severity;

pub mod kinds;

pub use collection::DiagnosticCollection;
pub use diagnostic::Diagnostic;
pub use extensions::DiagnosticExtensions;
pub use severity::DiagnosticSeverity;
