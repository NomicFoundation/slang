mod specifier;
#[path = "version.generated.rs"]
mod version;

pub use specifier::LanguageVersionSpecifier;
pub use version::{FromSemverError, LanguageVersion};

#[cfg(test)]
mod tests;
