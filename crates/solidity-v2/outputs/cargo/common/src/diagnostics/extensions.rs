use crate::diagnostics::severity::DiagnosticSeverity;

/// Metadata accessors shared by every diagnostic type in this module.
///
/// Leaf diagnostic structs own the canonical implementation of each method —
/// they decide the severity, the stable machine-readable code, and the
/// human-readable message.
///
/// Consumers that render diagnostics should prefer these accessors over
/// pattern-matching the enum variants directly, so that new variants added in
/// the future stay automatically covered.
pub trait DiagnosticExtensions {
    /// Returns the severity classification for this diagnostic.
    fn severity(&self) -> DiagnosticSeverity;

    /// Returns a stable machine-readable code identifying this diagnostic.
    /// Intended for matching, filtering, and linking to documentation.
    fn code(&self) -> &'static str;

    /// Returns a human-readable message describing this diagnostic.
    /// The message is produced on demand.
    fn message(&self) -> String;
}
