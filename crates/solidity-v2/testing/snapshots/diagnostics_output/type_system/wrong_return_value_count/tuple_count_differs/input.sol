// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a tuple returns as many values as it has components, whether
// written out or produced by a call.

contract C {
    function pair() internal pure returns (uint256, uint256) {
        return (1, 2);
    }

    function f() internal pure returns (uint256) {
        return (1, 2);
    }

    function g() internal pure returns (uint256, uint256) {
        return (1, 2, 3);
    }

    function h() internal pure returns (uint256) {
        return pair();
    }
}
