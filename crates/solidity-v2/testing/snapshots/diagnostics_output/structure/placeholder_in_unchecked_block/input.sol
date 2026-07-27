// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint counter;

    // A placeholder statement inside an `unchecked` block is not allowed.
    modifier m1() {
        unchecked {
            _;
        }
    }

    // Still flagged when nested deeper inside the `unchecked` block.
    modifier m2(uint x) {
        unchecked {
            if (x > 0) {
                _;
            }
        }
    }

    // A placeholder outside any `unchecked` block is fine, even alongside one.
    modifier m3() {
        unchecked {
            counter++;
        }
        _;
    }
}
