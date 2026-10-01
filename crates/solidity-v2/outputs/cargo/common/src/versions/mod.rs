mod default_evm_target;
mod specifier;
#[path = "version.generated.rs"]
mod version;

pub use specifier::LanguageVersionSpecifier;
pub use version::{LanguageVersion, LanguageVersionConversionError};

#[cfg(test)]
mod tests;
