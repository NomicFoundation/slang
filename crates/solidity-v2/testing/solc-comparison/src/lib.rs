//! Runs slang v2 against solc's `libsolidity` semantic tests — a large corpus
//! of known-valid Solidity — as a guard against new validations accidentally
//! rejecting valid code.
//!
//! The building blocks live in submodules: fetching each version's tests
//! ([`dataset`]), parsing the `isoltest` format ([`mod@test_case`]), running
//! slang ([`runner`]), and the checked-in per-version tallies
//! ([`results`]). [`suite::run`] ties them together, and the whole matrix runs
//! as the single test below.

use std::path::PathBuf;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;

pub mod dataset;
pub mod results;
pub mod runner;
pub mod suite;
pub mod test_case;

const CRATE_NAME: &str = "solidity_testing_solc_comparison";

/// Path to one of this crate's checked-in generated files, located via the
/// shared cargo-workspace helper (which resolves the crate's source directory
/// from the workspace manifest).
pub(crate) fn generated_file(name: &str) -> Result<PathBuf> {
    Ok(CargoWorkspace::locate_source_crate(CRATE_NAME)?.join(name))
}

/// Compiles every semantic test at every supported language version.
///
/// This is deliberately a single test rather than one per case: the ~60k cases
/// run in a couple of seconds in-process across rayon, so splitting them up
/// would only cost `nextest` tens of thousands of processes.
#[test]
#[ignore = "downloads solc's semanticTests; run it with `infra test solc-semantic`"]
fn solc_semantic_suite() -> Result<()> {
    suite::run()
}
