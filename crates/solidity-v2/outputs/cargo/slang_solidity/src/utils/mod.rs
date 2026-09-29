pub use slang_solidity_v2_common::evm_targets::{
    EvmTarget, EvmTargetConversionError, EvmTargetSpecifier,
};
pub use slang_solidity_v2_common::versions::{
    LanguageVersion, LanguageVersionConversionError, LanguageVersionSpecifier,
};

mod version_inference;

pub use version_inference::{VersionConstraint, infer_language_versions};
