// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // https://github.com/argotorg/solidity/blob/v0.8.27/test/libsolidity/syntaxTests/immutable/state_var_transient_data_location.sol
    // A state variable cannot be both `immutable` and `transient`.
    uint public immutable transient x;
}
