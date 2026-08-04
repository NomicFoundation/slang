//! Resolution of the per-test `.tests.config.json` files.

mod expected_solc_divergence;
mod selected_target;
mod selected_version;
mod single_target_all_versions;
mod single_version_all_targets;
mod test_config;
mod test_matrix;

pub use test_config::TestConfig;
pub use test_matrix::TestMatrix;
