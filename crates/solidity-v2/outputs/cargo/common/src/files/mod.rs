use std::fmt;
use std::sync::Arc;

use serde::Serialize;

/// Identifies a source file within a compilation unit.
///
/// Wraps the user-provided string identifier (typically a file path or URI) that is
/// used to reference files throughout the compilation pipeline and AST.
///
/// Stored as an [`Arc<str>`] so that cloning a `FileId` during analysis — which
/// happens for every file, import, and scope — is a cheap reference-count bump
/// rather than a heap allocation and string copy.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct FileId(Arc<str>);

impl FileId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for FileId {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<&str> for FileId {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
