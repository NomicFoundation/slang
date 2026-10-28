// SPDX-License-Identifier: MIT
pragma solidity *;

// The named return variable `N` shadows the constant even when declared after
// its use, so the length is not a constant.
contract C {
    uint256 constant N = 2;

    function f() public pure returns (uint256[N] memory arr, uint256 N) {}
}
