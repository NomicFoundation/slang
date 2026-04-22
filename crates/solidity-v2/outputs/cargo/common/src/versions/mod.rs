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

impl LanguageVersionSpecifier {
    pub fn contains(&self, other: LanguageVersion) -> bool {
        match self {
            LanguageVersionSpecifier::From { from } => other >= *from,
            LanguageVersionSpecifier::Till { till } => other < *till,
            LanguageVersionSpecifier::Range { from, till } => other >= *from && other < *till,
        }
    }
}

#[cfg(test)]
mod tests;
