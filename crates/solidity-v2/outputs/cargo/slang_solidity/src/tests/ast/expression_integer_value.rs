use num_bigint::BigInt;

use crate::define_fixture;

define_fixture!(
    FoldedConstants,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8;

contract Test {
    uint256 folded = 2 + 3;
    uint256 wide = 2**300 / 2**299;
    uint256 runtime = folded + 1;
}
"#,
);

#[test]
fn integer_value_folds_constant_expressions() {
    let unit = FoldedConstants::build_compilation_unit();
    let test = unit
        .find_contract_by_name("Test")
        .next()
        .expect("Test contract can be found");

    let state_variables = test.state_variables();
    assert_eq!(state_variables.len(), 3);
    let value_of = |index: usize| {
        state_variables[index]
            .value()
            .expect("has initializer")
            .integer_value()
    };

    // 2 + 3
    assert_eq!(value_of(0), Some(BigInt::from(5)));
    // 2**300 / 2**299: the intermediates exceed every machine integer type
    assert_eq!(value_of(1), Some(BigInt::from(2)));
    // reads a state variable, so it is not a compile-time constant
    assert_eq!(value_of(2), None);
}
