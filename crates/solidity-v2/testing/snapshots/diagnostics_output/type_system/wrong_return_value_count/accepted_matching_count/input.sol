// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: the values returned match the declared count, including a
// parenthesised single value and a call returning a tuple.

contract C {
    function pair() internal pure returns (uint256, uint256) {
        return (1, 2);
    }

    function f() internal pure returns (uint256) {
        return (1);
    }

    function g() internal pure returns (uint256, uint256) {
        return pair();
    }
}
