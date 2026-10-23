// SPDX-License-Identifier: MIT
pragma solidity *;

// The modifier parameter `N` shadows the constant, so the length is not a constant.
contract C {
    uint256 constant N = 2;

    modifier m(uint256 N) {
        uint256[N] memory arr;
        _;
    }
}
