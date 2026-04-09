#[path = "language_versions.generated.rs"]
mod language_versions;

pub use language_versions::{FromSemverError, LanguageVersion};

#[derive(Clone, Debug, PartialEq)]
pub enum LanguageVersionSpecifier {
    From {
        from: LanguageVersion,
    },
    Till {
        till: LanguageVersion,
    },
    Range {
        from: LanguageVersion,
        till: LanguageVersion,
    },
}

#[cfg(test)]
mod tests;
