use serde::Serialize;

/// The kind of catch clause a `try` statement may declare at most once.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum CatchClauseKind {
    /// A named `catch Error(...)` clause.
    Error,
    /// A named `catch Panic(...)` clause.
    Panic,
    /// A low-level clause without a selector name: `catch { ... }` or
    /// `catch (bytes ...) { ... }`.
    LowLevel,
}
