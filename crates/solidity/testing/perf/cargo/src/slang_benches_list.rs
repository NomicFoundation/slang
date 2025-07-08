/*
  A little hack to avoid repetion of code in the two settings where we run
  tests: ./benches/iai/main.rs and ./lib.rs

  __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
*/
slang_define_payload_tests!(flat_imports_mooniswap, "flat_imports_mooniswap");
slang_define_payload_tests!(
    circular_imports_weighted_pool,
    "circular_imports_weighted_pool"
);
slang_define_payload_tests!(protocol_uniswap, "protocol_uniswap");
slang_define_payload_tests!(protocol_multicall3, "protocol_multicall3");
slang_define_payload_tests!(protocol_create_x, "protocol_create_x");
slang_define_payload_tests!(
    protocol_ui_pool_data_provider_v3,
    "protocol_ui_pool_data_provider_v3"
);
slang_define_payload_tests!(deep_nesting_cooldogs, "deep_nesting_cooldogs");
slang_define_payload_tests!(largest_file_trivia_oslf, "largest_file_trivia_oslf");
slang_define_payload_tests!(median_file_safe_math, "median_file_safe_math");
slang_define_payload_tests!(
    three_quarters_file_merkle_proof,
    "three_quarters_file_merkle_proof"
);
