// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract C {
    // An unimplemented modifier must be marked `virtual`.
    modifier m1;

    // Marked `virtual`, so this is allowed.
    modifier m2 virtual;

    // Has an implementation body, so this is allowed.
    modifier m3() {
        _;
    }
}
