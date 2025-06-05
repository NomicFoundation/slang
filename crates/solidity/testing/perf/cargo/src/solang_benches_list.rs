// Most projects don't compile with solang; we test only those that don't fail.
// We place the marker to keep in sync for two reasons: to make sure we test eventually new versions of solang,
// and check if they make more tests pass.
//
// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
// __SLANG_INFRA_SOLANG_VERSION__ (check other projects)
solang_define_payload_test!(median_file_safe_math, "median_file_safe_math");
solang_define_payload_test!(
    three_quarters_file_merkle_proof,
    "three_quarters_file_merkle_proof"
);
