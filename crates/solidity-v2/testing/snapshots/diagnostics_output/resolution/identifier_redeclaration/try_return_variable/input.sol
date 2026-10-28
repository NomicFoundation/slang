// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() external pure returns (uint, uint) {}

    function f() public view {
        try this.g() returns (uint a, uint a) {} catch {}
    }
}
