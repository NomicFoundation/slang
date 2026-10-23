// SPDX-License-Identifier: MIT
pragma solidity *;

// Only the undeclared name in the constant's value is reported, not the
// array length that refers to the constant.
contract C {
    uint256 constant N = M;
    uint256[N] arr;
}
