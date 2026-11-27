// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a modifier has no return values, so a `return` in its body cannot
// carry one.

contract C {
    modifier m() {
        _;
        return 1;
    }
}
