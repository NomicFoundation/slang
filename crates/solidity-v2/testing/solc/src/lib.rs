//! Runs slang v2 against solc's `libsolidity` semantic tests — a large corpus
//! of known-valid Solidity — as a guard against new validations accidentally
//! rejecting valid code.
//!
//! The building blocks live in submodules: fetching each version's tests
//! ([`dataset`]), parsing the `isoltest` format ([`mod@test_case`]), running
//! slang ([`runner`]), and the checked-in per-version record
//! ([`results`]). [`suite::run`] ties them together, and the whole matrix runs
//! as the single test below.

pub mod dataset;
pub mod results;
pub mod runner;
pub mod suite;
pub mod test_case;

/// Compiles every semantic test at every supported language version.
///
/// This is deliberately a single test rather than one per case: the ~50k cases
/// run in a couple of seconds in-process across rayon, so splitting them up
/// would only cost `nextest` tens of thousands of processes.
///
/// Like the other suites that work off a downloaded corpus, the first run clones
/// `solc`'s repository into `target/`; every run after that is offline.
#[test]
fn solc_semantic_suite() -> anyhow::Result<()> {
    suite::run()
}
