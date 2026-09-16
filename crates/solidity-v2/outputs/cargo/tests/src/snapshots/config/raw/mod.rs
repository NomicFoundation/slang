//! The `.tests.config.json` fields exactly as they are written in a file.
//!
//! Each module owns one field: its as-written form, and the `TryFrom` impl that
//! validates it and converts it into what [`super::TestConfig`] holds.

pub(crate) mod evm_target_override;
pub(crate) mod expected_solc_divergence;
pub(crate) mod language_version_override;
pub(crate) mod test_config;
