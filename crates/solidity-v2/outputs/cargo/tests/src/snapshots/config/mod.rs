//! Resolution of the per-test `.tests.config.json` files.

mod raw;
mod test_case;
mod test_config;

pub use test_case::TestCase;
pub use test_config::TestConfig;
