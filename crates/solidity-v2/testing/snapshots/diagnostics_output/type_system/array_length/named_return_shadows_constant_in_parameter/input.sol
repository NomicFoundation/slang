// SPDX-License-Identifier: MIT
pragma solidity *;

// Parameters and returns share a scope, so the named return variable `N`
// shadows the constant in a parameter type.
contract C {
    uint256 constant N = 2;

    function f(uint256[N] memory arr) public pure returns (uint256 N) {}
}
