// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g(uint256) external pure returns (uint256) {}

    function f() public view {
        // Invalid: the called expression of a try statement is a read position.
        try this.g((1, )) returns (uint256) {} catch {}
    }
}
