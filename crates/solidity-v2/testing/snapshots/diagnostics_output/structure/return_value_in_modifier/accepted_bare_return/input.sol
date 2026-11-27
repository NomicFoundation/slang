// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a bare `return` leaves a modifier early, wherever it appears in its
// body.

contract C {
    modifier m1() {
        _;
        return;
    }

    modifier m2(bool c) {
        if (c) {
            return;
        }
        _;
    }
}
