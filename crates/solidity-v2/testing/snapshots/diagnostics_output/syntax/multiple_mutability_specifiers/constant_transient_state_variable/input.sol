// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // https://github.com/argotorg/solidity/blob/v0.8.27/test/libsolidity/syntaxTests/constants/constant_transient_state_variable.sol
    // A state variable cannot be both `constant` and `transient`.
    int constant public transient x = 0;
}
