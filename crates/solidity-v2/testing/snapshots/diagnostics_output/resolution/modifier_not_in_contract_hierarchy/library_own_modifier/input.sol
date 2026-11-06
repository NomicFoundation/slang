// SPDX-License-Identifier: MIT
pragma solidity *;

// A library may invoke its own modifiers.
library L {
    modifier m() {
        _;
    }

    function f() internal m {}
}
