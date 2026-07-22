// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract C {
    // An implemented modifier with an empty body has no placeholder.
    modifier m1() {}

    // An implemented modifier whose body has statements but no placeholder.
    modifier m2() {
        require(msg.sender != address(0));
    }

    // A placeholder statement makes the modifier valid.
    modifier m3() {
        _;
    }

    // A placeholder nested within control flow still counts.
    modifier m4(uint x) {
        if (x > 0) {
            _;
        }
    }

    // A modifier without an implementation body is not subject to this check.
    modifier m5() virtual;
}
