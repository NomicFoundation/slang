// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a bare `return` leaves a function without return values or a
// modifier, including from a `try` clause whose own `returns` are not the
// function's.

contract C {
    function g() external pure returns (uint256) {
        return 1;
    }

    modifier m() {
        _;
        return;
    }

    function f() external view m {
        try this.g() returns (uint256) {
            return;
        } catch {
            return;
        }
    }
}
