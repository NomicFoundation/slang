// SPDX-License-Identifier: MIT
pragma solidity *;

// The loop variable `N` shadows the constant, so the length is not a constant.
contract C {
    uint256 constant N = 2;

    function f() public pure {
        for (uint256 N = 0; N < 1; N++) {
            uint256[N] memory arr;
        }
    }
}
