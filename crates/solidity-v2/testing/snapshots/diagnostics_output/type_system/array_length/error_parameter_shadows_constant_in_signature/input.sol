// SPDX-License-Identifier: MIT
pragma solidity *;

// The error parameter `N` shadows the constant in a sibling parameter type, so
// the length is not a constant.
contract C {
    uint256 constant N = 2;

    error E(uint256 N, uint256[N] a);
}
