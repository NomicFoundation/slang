// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a single value only matches a function that declares exactly one
// return value.

contract C {
    function f() internal pure returns (uint256, uint256) {
        return 1;
    }

    function g() internal pure {
        return 1;
    }
}
