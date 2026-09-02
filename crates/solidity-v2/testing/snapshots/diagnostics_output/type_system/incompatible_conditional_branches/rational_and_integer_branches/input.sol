// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A non-integer rational has no common type with an integer literal.
    function f() public pure returns (uint256) {
        return true ? 0.5 : 1;
    }
}
