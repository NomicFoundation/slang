// SPDX-License-Identifier: MIT
pragma solidity *;

// A modifier is not a type.
contract C {
    modifier m() {
        _;
    }

    function f() public {
        m y;
    }
}
