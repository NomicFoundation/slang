#[path = "versions.generated.rs"]
pub mod versions;

pub use versions::{FromVersionErrors, LanguageVersion};

#[cfg(test)]
mod tests;
