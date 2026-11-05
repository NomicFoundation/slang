// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure returns (uint256) {
        // A string literal has no members.
        return "abc".length;
    }
}
