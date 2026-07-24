//! Runs slang v2 against solc's `libsolidity` semantic tests. The building
//! blocks it uses live in submodules — fetching each version's tests
//! ([`dataset`]), parsing the `isoltest` format ([`mod@test_case`]), running
//! slang ([`runner`]), and the checked-in baseline ([`baseline`]) — and the
//! harness that ties them together is `tests/semantic_tests.rs`.

#[cfg(test)]
use datatest_stable as _;

pub mod baseline;
pub mod dataset;
pub mod runner;
pub mod test_case;
