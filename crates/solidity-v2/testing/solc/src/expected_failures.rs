use std::mem::take;

use anyhow::{Result, bail};
use slang_solidity_v2_common::versions::{LanguageVersion, LanguageVersionSpecifier};

use crate::results::VersionRun;

/// A set of expected failures that share one justification.
struct ExpectedFailures {
    /// Why `slang` is intentionally stricter (or looser) than `solc` for every
    /// case below.
    reason: &'static str,

    /// The failures this reason accounts for.
    cases: &'static [ExpectedCase],
}

/// A single failure covered by an [`ExpectedFailures`] reason.
struct ExpectedCase {
    /// A single test's path relative to `semanticTests` (e.g.
    /// `revertStrings/empty_v1.sol`).
    test_path: &'static str,

    /// The versions the test is expected to fail at.
    versions: LanguageVersionSpecifier,
}

impl ExpectedCase {
    /// Create an `ExpectedCase` that covers the given `test_path` from the given `version`.
    const fn from(test_path: &'static str, version: LanguageVersion) -> Self {
        Self {
            test_path,
            versions: LanguageVersionSpecifier::from(version),
        }
    }

    /// Create an `ExpectedCase` that covers the given `test_path` from the given `version` range.
    const fn range(test_path: &'static str, from: LanguageVersion, till: LanguageVersion) -> Self {
        Self {
            test_path,
            versions: LanguageVersionSpecifier::range(from, till),
        }
    }

    fn matches(&self, version: LanguageVersion, test_path: &str) -> bool {
        test_path == self.test_path && self.versions.contains(version)
    }
}

/// Fails when a declared case no longer describes a failing test, so that a
/// case outliving the divergence it stands for is caught rather than silently
/// ignored.
///
/// Reads what [`partition`] sorted out, so it runs after it.
pub fn check_stale(runs: &[VersionRun]) -> Result<()> {
    check_stale_against(EXPECTED_FAILURES, runs)
}

fn check_stale_against(table: &[ExpectedFailures], runs: &[VersionRun]) -> Result<()> {
    let mut stale = Vec::new();

    for expected in table {
        for case in expected.cases {
            let missing_versions: Vec<String> = runs
                .iter()
                .filter(|run| case.versions.contains(run.version))
                .filter(|run| {
                    !run.expected_failures
                        .iter()
                        .any(|failure| failure.test_path == case.test_path)
                })
                .map(|run| run.version.to_string())
                .collect();

            if !missing_versions.is_empty() {
                stale.push(format!(
                    "  - '{test_path}' did not fail at {versions}\n    declared for: {reason}",
                    test_path = case.test_path,
                    versions = missing_versions.join(", "),
                    reason = expected.reason,
                ));
            }
        }
    }

    if stale.is_empty() {
        return Ok(());
    }

    bail!(
        "Some expected failures no longer describe a failing test. Narrow \
         each case to the versions that still fail, or remove it from \
         `EXPECTED_FAILURES`:\n{stale}",
        stale = stale.join("\n"),
    );
}

/// Sorts each run's failures into the ones a declared case stands behind and
/// the ones nothing accounts for, moving the former into `expected_failures`.
pub fn partition(runs: &mut [VersionRun]) {
    partition_against(EXPECTED_FAILURES, runs);
}

fn partition_against(table: &[ExpectedFailures], runs: &mut [VersionRun]) {
    for run in runs {
        let version = run.version;

        let (expected, unexpected) =
            take(&mut run.unexpected_failures)
                .into_iter()
                .partition(|failure| {
                    table
                        .iter()
                        .flat_map(|expected| expected.cases)
                        .any(|case| case.matches(version, &failure.test_path))
                });

        run.expected_failures = expected;
        run.unexpected_failures = unexpected;
    }
}

#[cfg(test)]
mod tests {
    #[allow(clippy::enum_glob_use)]
    use LanguageVersion::*;

    use super::*;
    use crate::results::Failure;

    const TABLE: &[ExpectedFailures] = &[ExpectedFailures {
        reason: "A reason.",
        cases: &[ExpectedCase::range("a.sol", V0_8_0, V0_8_5)],
    }];

    /// One version's run, with the tests that failed at it.
    fn run(version: LanguageVersion, failing: &[&str]) -> VersionRun {
        VersionRun {
            version,
            commit: "0000000".to_owned(),
            executed: failing.len(),
            unexpected_failures: failing
                .iter()
                .map(|path| Failure {
                    version,
                    test_path: (*path).to_owned(),
                    diagnostics: Vec::new(),
                })
                .collect(),
            expected_failures: Vec::new(),
        }
    }

    /// The paths sorted into the expected pile.
    fn expected(run: &VersionRun) -> Vec<&str> {
        run.expected_failures
            .iter()
            .map(|failure| failure.test_path.as_str())
            .collect()
    }

    /// The paths still left unexpected.
    fn unexpected(run: &VersionRun) -> Vec<&str> {
        run.unexpected_failures
            .iter()
            .map(|failure| failure.test_path.as_str())
            .collect()
    }

    /// An expected failure moves to `expected_failures`; every other failure
    /// stays put.
    #[test]
    fn expected_failures_are_partitioned_from_the_rest() {
        let mut runs = vec![run(V0_8_0, &["a.sol", "b.sol"])];

        partition_against(TABLE, &mut runs);
        check_stale_against(TABLE, &runs).unwrap();

        assert_eq!(expected(&runs[0]), ["a.sol"]);
        assert_eq!(unexpected(&runs[0]), ["b.sol"]);
    }

    /// The case this per-version accounting exists for: still failing
    /// somewhere, but no longer everywhere it claims.
    #[test]
    fn a_case_that_stopped_failing_at_one_version_is_stale() {
        let mut runs = vec![run(V0_8_0, &[]), run(V0_8_1, &["a.sol"])];

        partition_against(TABLE, &mut runs);
        let error = check_stale_against(TABLE, &runs).unwrap_err().to_string();

        assert!(error.contains("'a.sol' did not fail at 0.8.0"), "{error}");
        assert!(!error.contains("0.8.1"), "{error}");
    }

    /// A case is held to its range and nothing wider: a version the test
    /// passes at, but that the range excludes, isn't drift.
    #[test]
    fn a_version_outside_the_range_is_not_held_against_the_case() {
        let mut runs = vec![
            run(V0_8_4, &["a.sol"]),
            // A range's upper bound is exclusive, so it doesn't claim 0.8.5.
            run(V0_8_5, &[]),
        ];

        partition_against(TABLE, &mut runs);
        check_stale_against(TABLE, &runs).unwrap();
    }

    /// Paths are whole names, never prefixes: neither the directory holding
    /// the test nor a sibling under it is covered.
    #[test]
    fn a_case_covers_exactly_the_test_it_names() {
        let mut runs = vec![run(V0_8_0, &["a.sol/nested.sol"])];

        partition_against(TABLE, &mut runs);
        let error = check_stale_against(TABLE, &runs).unwrap_err().to_string();

        assert!(error.contains("'a.sol' did not fail at 0.8.0"), "{error}");
        assert!(expected(&runs[0]).is_empty());
        assert_eq!(unexpected(&runs[0]), ["a.sol/nested.sol"]);
    }
}

#[allow(clippy::items_after_test_module)]
/// Every failure we currently stand behind.
const EXPECTED_FAILURES: &[ExpectedFailures] = {
    // The table names a version on nearly every line; the `LanguageVersion::` prefix on each
    // would drown out the paths that are the point of it.
    #[allow(clippy::enum_glob_use)]
    use LanguageVersion::*;

    &[
        ExpectedFailures {
            reason: r"`pragma experimental solidity` selects solc's experimental Solidity \
                       front-end, slang doesn't support experimental solidity.",
            cases: &[
                ExpectedCase::from("experimental/stub.sol", V0_8_24),
                ExpectedCase::from("experimental/type_class.sol", V0_8_24),
            ],
        },
        ExpectedFailures {
            reason: r"`pragma abicoder v1` selects solc's original ABI coder, which \
                       slang deliberately does not implement.",
            cases: &[
                ExpectedCase::range(
                    "abiEncodeDecode/abi_encode_empty_string_v1.sol",
                    V0_8_29,
                    V0_8_36,
                ),
                ExpectedCase::range(
                    "abiEncodeDecode/abi_encode_with_selector.sol",
                    V0_8_29,
                    V0_8_36,
                ),
                ExpectedCase::range(
                    "abiEncodeDecode/abi_encode_with_signature.sol",
                    V0_8_29,
                    V0_8_36,
                ),
                ExpectedCase::range("abiEncoderV1/abi_encode_empty_string.sol", V0_8_5, V0_8_36),
                ExpectedCase::range("abiEncoderV1/bool_out_of_bounds.sol", V0_8_5, V0_8_36),
                ExpectedCase::range(
                    "abiEncoderV1/calldata_bytes_bytes32_arrays.sol",
                    V0_8_16,
                    V0_8_36,
                ),
                ExpectedCase::range("abiEncoderV1/cleanup/cleanup.sol", V0_8_5, V0_8_36),
                ExpectedCase::range("abiEncoderV1/enums.sol", V0_8_5, V0_8_36),
                ExpectedCase::range(
                    "abiEncoderV1/memory_dynamic_array_and_calldata_bytes.sol",
                    V0_8_16,
                    V0_8_36,
                ),
                ExpectedCase::range(
                    "abiEncoderV2/abi_encode_v2_in_function_inherited_in_v1_contract.sol",
                    V0_8_5,
                    V0_8_36,
                ),
                ExpectedCase::range(
                    "abiEncoderV2/abi_encode_v2_in_modifier_used_in_v1_contract.sol",
                    V0_8_5,
                    V0_8_36,
                ),
                ExpectedCase::from("abicoder/abi_encode_empty_string_v1.sol", V0_8_36),
                ExpectedCase::from(
                    "abicoder/abi_encode_memory_dynamic_array_and_calldata_bytes_v1.sol",
                    V0_8_36,
                ),
                ExpectedCase::from(
                    "abicoder/abi_encode_v2_in_function_inherited_in_v1_contract.sol",
                    V0_8_36,
                ),
                ExpectedCase::from(
                    "abicoder/abi_encode_v2_in_modifier_used_in_v1_contract.sol",
                    V0_8_36,
                ),
                ExpectedCase::from("abicoder/abi_encode_with_selector_v1.sol", V0_8_36),
                ExpectedCase::from("abicoder/abi_encode_with_signature_v1.sol", V0_8_36),
                ExpectedCase::from(
                    "abicoder/calldataDecoding/array/calldata_bytes_bytes32_arrays_v1.sol",
                    V0_8_36,
                ),
                ExpectedCase::from("abicoder/cleanup/bool_v1.sol", V0_8_36),
                ExpectedCase::from("abicoder/cleanup/cleanup_v1.sol", V0_8_36),
                ExpectedCase::from("abicoder/cleanup/enum_v1.sol", V0_8_36),
                ExpectedCase::from(
                    "abicoder/validation/array_exceeds_calldatasize_v1.sol",
                    V0_8_36,
                ),
                ExpectedCase::from(
                    "abicoder/validation/array_exceeds_size_limit_for_calldata_types_v1.sol",
                    V0_8_36,
                ),
                ExpectedCase::range(
                    "abiencodedecode/abi_encode_empty_string_v1.sol",
                    V0_8_5,
                    V0_8_29,
                ),
                ExpectedCase::range(
                    "abiencodedecode/abi_encode_with_selector.sol",
                    V0_8_1,
                    V0_8_29,
                ),
                ExpectedCase::range(
                    "abiencodedecode/abi_encode_with_signature.sol",
                    V0_8_1,
                    V0_8_29,
                ),
                ExpectedCase::from("arithmetics/checked_add_v1.sol", V0_8_5),
                ExpectedCase::from("cleanup/bool_conversion_v1.sol", V0_8_5),
                ExpectedCase::from("cleanup/cleanup_address_types_v1.sol", V0_8_5),
                ExpectedCase::from("cleanup/cleanup_bytes_types_v1.sol", V0_8_5),
                ExpectedCase::from("operators/shifts/shift_right_garbled_signed_v1.sol", V0_8_5),
                ExpectedCase::from("operators/shifts/shift_right_garbled_v1.sol", V0_8_5),
                ExpectedCase::from(
                    "operators/shifts/shift_right_negative_lvalue_signextend_int16_v1.sol",
                    V0_8_5,
                ),
                ExpectedCase::from(
                    "operators/shifts/shift_right_negative_lvalue_signextend_int32_v1.sol",
                    V0_8_5,
                ),
                ExpectedCase::from(
                    "operators/shifts/shift_right_negative_lvalue_signextend_int8_v1.sol",
                    V0_8_5,
                ),
                ExpectedCase::from("revertStrings/calldata_too_short_v1.sol", V0_8_5),
                ExpectedCase::from("revertStrings/empty_v1.sol", V0_8_0),
                ExpectedCase::from("revertStrings/enum_v1.sol", V0_8_5),
                ExpectedCase::from("revertStrings/function_entry_checks_v1.sol", V0_8_0),
                ExpectedCase::from("revertStrings/invalid_abi_decoding_calldata_v1.sol", V0_8_5),
                ExpectedCase::from("revertStrings/invalid_abi_decoding_memory_v1.sol", V0_8_5),
                ExpectedCase::from("types/mapping_enum_key_getter_v1.sol", V0_8_5),
                ExpectedCase::from("types/mapping_enum_key_library_v1.sol", V0_8_5),
                ExpectedCase::from("types/mapping_enum_key_v1.sol", V0_8_5),
                ExpectedCase::from(
                    "userDefinedValueType/assembly_access_bytes2_abicoder_v1.sol",
                    V0_8_9,
                ),
                ExpectedCase::from("userDefinedValueType/cleanup_abicoderv1.sol", V0_8_8),
                ExpectedCase::from("userDefinedValueType/conversion_abicoderv1.sol", V0_8_8),
            ],
        },
        ExpectedFailures {
            reason: r"`EVMVersion: =@future` asks for the next EVM version, which by \
                       definition has not been released. Slang doesn't support this.",
            cases: &[ExpectedCase::from(
                "isoltestTesting/future_evm_version_smoke_test.sol",
                V0_8_35,
            )],
        },
        ExpectedFailures {
            reason: r"solc allows to refer module aliases in ternary expressions as long as they \
                      refer to the same module, `(flag ? M : M).C`. Slang does not allow this.",
            cases: &[ExpectedCase::from(
                "expressions/module_from_ternary_expression.sol",
                V0_8_21,
            )],
        },
    ]
};
