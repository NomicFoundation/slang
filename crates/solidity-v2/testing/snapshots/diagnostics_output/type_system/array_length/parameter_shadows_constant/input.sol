// SPDX-License-Identifier: MIT
pragma solidity *;

// The parameter `N` shadows the constant, so the length is not a constant.
contract C {
    uint256 constant N = 2;

    function f(uint256 N) public pure {
        uint256[N] memory arr;
    }
}
