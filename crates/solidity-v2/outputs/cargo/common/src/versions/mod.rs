#[path = "language_versions.generated.rs"]
mod language_versions;

pub use language_versions::{FromSemverError, LanguageVersion};

#[cfg(test)]
mod tests;
