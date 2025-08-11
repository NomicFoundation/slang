/*
  A little hack to avoid repetion of code in the two settings where we run
  tests: ./benches/iai/main.rs and ./lib.rs

  __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
*/
// fails with parsing error (requires Solidity 0.6)
// solar_define_payload_test!(flat_imports_mooniswap, "flat_imports_mooniswap");
solar_define_payload_test!(
    circular_imports_weighted_pool,
    "circular_imports_weighted_pool"
);
solar_define_payload_test!(protocol_uniswap, "protocol_uniswap");
solar_define_payload_test!(protocol_multicall3, "protocol_multicall3");
solar_define_payload_test!(protocol_create_x, "protocol_create_x");
solar_define_payload_test!(
    protocol_ui_pool_data_provider_v3,
    "protocol_ui_pool_data_provider_v3"
);
solar_define_payload_test!(deep_nesting_cooldogs, "deep_nesting_cooldogs");
solar_define_payload_test!(largest_file_trivia_oslf, "largest_file_trivia_oslf");
solar_define_payload_test!(median_file_safe_math, "median_file_safe_math");
solar_define_payload_test!(
    three_quarters_file_merkle_proof,
    "three_quarters_file_merkle_proof"
);
