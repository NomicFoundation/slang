// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    modifier m() {
        _;
    }

    // Invoking the modifier from the function header is fine.
    function f() public m {
        // A statement is not exempt: naming the modifier is still an error.
        m;
    }
}
