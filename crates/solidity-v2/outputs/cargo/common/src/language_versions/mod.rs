#[path = "versions.generated.rs"]
pub mod versions;

pub use versions::LanguageVersion;

#[cfg(test)]
mod tests;
