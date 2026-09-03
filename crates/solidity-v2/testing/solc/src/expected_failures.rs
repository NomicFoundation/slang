use anyhow::{Result, bail};
use slang_solidity_v2_common::versions::{LanguageVersion, LanguageVersionSpecifier};

use crate::results::{AllFailures, SplitFailures, VersionRun};

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
    fn matches(&self, version: LanguageVersion, test_path: &str) -> bool {
        test_path == self.test_path && self.versions.contains(version)
    }
}

/// Splits each run's expected failures out into a count, which keeps them out
/// of the generated results file.
///
/// Also check that every expected failure is still failing.
///
/// Return the new runs and a `Result` containing the error, if any.
pub fn split_and_check(
    runs: Vec<VersionRun<AllFailures>>,
) -> (Vec<VersionRun<SplitFailures>>, Result<()>) {
    split_and_check_against(EXPECTED_FAILURES, runs)
}

fn split_and_check_against(
    table: &[ExpectedFailures],
    runs: Vec<VersionRun<AllFailures>>,
) -> (Vec<VersionRun<SplitFailures>>, Result<()>) {
    let stale_result = check_stale(table, &runs);

    let runs = runs
        .into_iter()
        .map(|run| {
            let version = run.version;

            run.map_failures(|all| split_failures(table, version, all))
        })
        .collect();

    (runs, stale_result)
}

fn split_failures(
    table: &[ExpectedFailures],
    version: LanguageVersion,
    AllFailures(mut unexpected): AllFailures,
) -> SplitFailures {
    let failed = unexpected.len();

    unexpected.retain(|failure| {
        !table
            .iter()
            .flat_map(|expected| expected.cases)
            .any(|case| case.matches(version, &failure.test_path))
    });

    SplitFailures {
        expected: failed - unexpected.len(),
        unexpected,
    }
}

fn check_stale(table: &[ExpectedFailures], runs: &[VersionRun<AllFailures>]) -> Result<()> {
    let mut stale = Vec::new();

    for expected in table {
        for case in expected.cases {
            let missing_versions: Vec<String> = runs
                .iter()
                .filter(|run| case.versions.contains(run.version))
                .filter(|run| {
                    !run.failures
                        .0
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::Failure;

    const TABLE: &[ExpectedFailures] = &[ExpectedFailures {
        reason: "A reason.",
        cases: &[ExpectedCase {
            test_path: "a.sol",
            versions: LanguageVersionSpecifier::till(LanguageVersion::V0_8_5),
        }],
    }];

    /// One version's run, with the tests that failed at it.
    fn run(version: LanguageVersion, failing: &[&str]) -> VersionRun<AllFailures> {
        VersionRun {
            version,
            commit: "0000000".to_owned(),
            executed: failing.len(),
            failures: AllFailures(
                failing
                    .iter()
                    .map(|path| Failure {
                        version,
                        test_path: (*path).to_owned(),
                        diagnostics: Vec::new(),
                    })
                    .collect(),
            ),
        }
    }

    /// The paths still left unexpected.
    fn unexpected(run: &VersionRun<SplitFailures>) -> Vec<&str> {
        run.failures
            .unexpected
            .iter()
            .map(|failure| failure.test_path.as_str())
            .collect()
    }

    /// An expected failure is counted under `expected` and dropped from
    /// `unexpected`; every other failure stays.
    #[test]
    fn expected_failures_are_split_from_the_rest() {
        let runs = vec![run(LanguageVersion::V0_8_0, &["a.sol", "b.sol"])];

        let (runs, stale) = split_and_check_against(TABLE, runs);

        stale.unwrap();

        assert_eq!(runs[0].failures.expected, 1);
        assert_eq!(unexpected(&runs[0]), ["b.sol"]);
    }

    /// The case this per-version accounting exists for: still failing
    /// somewhere, but no longer everywhere it claims.
    #[test]
    fn a_case_that_stopped_failing_at_one_version_is_stale() {
        let runs = vec![
            run(LanguageVersion::V0_8_0, &[]),
            run(LanguageVersion::V0_8_1, &["a.sol"]),
        ];

        let (_, stale) = split_and_check_against(TABLE, runs);
        let error = stale.unwrap_err().to_string();

        assert!(error.contains("'a.sol' did not fail at 0.8.0"), "{error}");
        assert!(!error.contains("0.8.1"), "{error}");
    }

    /// A case is held to its range and nothing wider: a version the test
    /// passes at, but that the range excludes, isn't drift.
    #[test]
    fn a_version_outside_the_range_is_not_held_against_the_case() {
        let runs = vec![
            run(LanguageVersion::V0_8_4, &["a.sol"]),
            // `Till` is exclusive, so the range doesn't claim 0.8.5.
            run(LanguageVersion::V0_8_5, &[]),
        ];

        let (_, stale) = split_and_check_against(TABLE, runs);

        stale.unwrap();
    }

    /// Paths are whole names, never prefixes: neither the directory holding
    /// the test nor a sibling under it is covered.
    #[test]
    fn a_case_covers_exactly_the_test_it_names() {
        let runs = vec![run(LanguageVersion::V0_8_0, &["a.sol/nested.sol"])];

        let (runs, stale) = split_and_check_against(TABLE, runs);
        let error = stale.unwrap_err().to_string();

        assert!(error.contains("'a.sol' did not fail at 0.8.0"), "{error}");
        assert_eq!(runs[0].failures.expected, 0);
        assert_eq!(unexpected(&runs[0]), ["a.sol/nested.sol"]);
    }
}

#[allow(clippy::items_after_test_module)]
/// Every failure we currently stand behind.
const EXPECTED_FAILURES: &[ExpectedFailures] = &[
    ExpectedFailures {
        reason: r"`pragma experimental solidity` selects solc's experimental Solidity \
             front-end, slang doesn't support experimental solidity.",
        cases: &[
            ExpectedCase {
                test_path: "experimental/stub.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
            },
            ExpectedCase {
                test_path: "experimental/type_class.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_24),
            },
        ],
    },
    ExpectedFailures {
        reason: r"`pragma abicoder v1` selects solc's original ABI coder, which \
                 slang deliberately does not implement.",
        cases: &[
            ExpectedCase {
                test_path: "abiEncodeDecode/abi_encode_empty_string_v1.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_29,
                    LanguageVersion::V0_8_36,
                ),
            },
            ExpectedCase {
                test_path: "abiEncodeDecode/abi_encode_with_selector.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_29,
                    LanguageVersion::V0_8_36,
                ),
            },
            ExpectedCase {
                test_path: "abiEncodeDecode/abi_encode_with_signature.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_29,
                    LanguageVersion::V0_8_36,
                ),
            },
            ExpectedCase {
                test_path: "abiEncoderV1/abi_encode_empty_string.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_5,
                    LanguageVersion::V0_8_36,
                ),
            },
            ExpectedCase {
                test_path: "abiEncoderV1/bool_out_of_bounds.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_5,
                    LanguageVersion::V0_8_36,
                ),
            },
            ExpectedCase {
                test_path: "abiEncoderV1/calldata_bytes_bytes32_arrays.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_16,
                    LanguageVersion::V0_8_36,
                ),
            },
            ExpectedCase {
                test_path: "abiEncoderV1/cleanup/cleanup.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_5,
                    LanguageVersion::V0_8_36,
                ),
            },
            ExpectedCase {
                test_path: "abiEncoderV1/enums.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_5,
                    LanguageVersion::V0_8_36,
                ),
            },
            ExpectedCase {
                test_path: "abiEncoderV1/memory_dynamic_array_and_calldata_bytes.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_16,
                    LanguageVersion::V0_8_36,
                ),
            },
            ExpectedCase {
                test_path: "abiEncoderV2/abi_encode_v2_in_function_inherited_in_v1_contract.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_5,
                    LanguageVersion::V0_8_36,
                ),
            },
            ExpectedCase {
                test_path: "abiEncoderV2/abi_encode_v2_in_modifier_used_in_v1_contract.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_5,
                    LanguageVersion::V0_8_36,
                ),
            },
            ExpectedCase {
                test_path: "abicoder/abi_encode_empty_string_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_36),
            },
            ExpectedCase {
                test_path: "abicoder/abi_encode_memory_dynamic_array_and_calldata_bytes_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_36),
            },
            ExpectedCase {
                test_path: "abicoder/abi_encode_v2_in_function_inherited_in_v1_contract.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_36),
            },
            ExpectedCase {
                test_path: "abicoder/abi_encode_v2_in_modifier_used_in_v1_contract.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_36),
            },
            ExpectedCase {
                test_path: "abicoder/abi_encode_with_selector_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_36),
            },
            ExpectedCase {
                test_path: "abicoder/abi_encode_with_signature_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_36),
            },
            ExpectedCase {
                test_path: "abicoder/calldataDecoding/array/calldata_bytes_bytes32_arrays_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_36),
            },
            ExpectedCase {
                test_path: "abicoder/cleanup/bool_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_36),
            },
            ExpectedCase {
                test_path: "abicoder/cleanup/cleanup_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_36),
            },
            ExpectedCase {
                test_path: "abicoder/cleanup/enum_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_36),
            },
            ExpectedCase {
                test_path: "abicoder/validation/array_exceeds_calldatasize_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_36),
            },
            ExpectedCase {
                test_path: "abicoder/validation/array_exceeds_size_limit_for_calldata_types_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_36),
            },
            ExpectedCase {
                test_path: "abiencodedecode/abi_encode_empty_string_v1.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_5,
                    LanguageVersion::V0_8_29,
                ),
            },
            ExpectedCase {
                test_path: "abiencodedecode/abi_encode_with_selector.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_1,
                    LanguageVersion::V0_8_29,
                ),
            },
            ExpectedCase {
                test_path: "abiencodedecode/abi_encode_with_signature.sol",
                versions: LanguageVersionSpecifier::range(
                    LanguageVersion::V0_8_1,
                    LanguageVersion::V0_8_29,
                ),
            },
            ExpectedCase {
                test_path: "arithmetics/checked_add_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "cleanup/bool_conversion_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "cleanup/cleanup_address_types_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "cleanup/cleanup_bytes_types_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "operators/shifts/shift_right_garbled_signed_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "operators/shifts/shift_right_garbled_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "operators/shifts/shift_right_negative_lvalue_signextend_int16_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "operators/shifts/shift_right_negative_lvalue_signextend_int32_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "operators/shifts/shift_right_negative_lvalue_signextend_int8_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "revertStrings/calldata_too_short_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "revertStrings/empty_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_0),
            },
            ExpectedCase {
                test_path: "revertStrings/enum_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "revertStrings/function_entry_checks_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_0),
            },
            ExpectedCase {
                test_path: "revertStrings/invalid_abi_decoding_calldata_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "revertStrings/invalid_abi_decoding_memory_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "types/mapping_enum_key_getter_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "types/mapping_enum_key_library_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "types/mapping_enum_key_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_5),
            },
            ExpectedCase {
                test_path: "userDefinedValueType/assembly_access_bytes2_abicoder_v1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_9),
            },
            ExpectedCase {
                test_path: "userDefinedValueType/cleanup_abicoderv1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_8),
            },
            ExpectedCase {
                test_path: "userDefinedValueType/conversion_abicoderv1.sol",
                versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_8),
            },
        ],
    },
    ExpectedFailures {
        reason: r"`EVMVersion: =@future` asks for the next EVM version, which by \
                 definition has not been released. Slang doesn't support this.",
        cases: &[ExpectedCase {
            test_path: "isoltestTesting/future_evm_version_smoke_test.sol",
            versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_35),
        }],
    },
];
