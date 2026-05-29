use crate::define_fixture;

define_fixture!(
    NumberLiterals,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract NumberLiterals {
    // Hex literals
    uint constant HEX_SMALL = 0xff;
    uint constant HEX_MIXED_CASE = 0xDeAdBeEf;
    uint constant HEX_WITH_SEP = 0x1234_5678;
    uint constant HEX_ZERO = 0x0;

    // Address literal: a hex literal of exactly 40 digits is typed as an address.
    address constant ADDRESS_BEEF = 0x000000000000000000000000000000000000beef;

    // Decimal integer literals
    uint constant DEC_ZERO = 0;
    uint constant DEC_INT = 42;
    uint constant DEC_WITH_SEP = 1_000_000;

    // Decimal with scientific notation
    uint constant DEC_EXP_POS = 1e3;
    uint constant DEC_EXP_NEG_INT = 1500e-2;

    // Decimal with units
    uint constant ONE_WEI = 1 wei;
    uint constant ONE_GWEI = 1 gwei;
    uint constant ONE_ETHER = 1 ether;
    uint constant HALF_ETHER = 0.5 ether;
    uint constant ONE_HOUR = 1 hours;

    // Rational decimal literals folded to integers through multiplication
    uint constant FROM_HALF = 0.5 * 4;
    uint constant FROM_ONE_AND_HALF = 1.5 * 2;
}
"#,
);
