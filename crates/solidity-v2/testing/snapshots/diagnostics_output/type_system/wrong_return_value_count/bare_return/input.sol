// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a bare `return` gives no values, so it cannot leave a function
// that declares return values, even named ones.

contract C {
    function f() internal pure returns (uint256) {
        return;
    }

    function g() internal pure returns (uint256 a) {
        a = 1;
        return;
    }
}
