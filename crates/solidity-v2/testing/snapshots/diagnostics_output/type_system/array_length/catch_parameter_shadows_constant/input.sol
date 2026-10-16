// SPDX-License-Identifier: MIT
pragma solidity *;

// The catch parameter `N` shadows the constant, so the length is not a constant.
contract C {
    uint256 constant N = 2;

    function g() external pure {}

    function f() public view {
        try this.g() {} catch Panic(uint256 N) {
            uint256[N] memory arr;
        }
    }
}
