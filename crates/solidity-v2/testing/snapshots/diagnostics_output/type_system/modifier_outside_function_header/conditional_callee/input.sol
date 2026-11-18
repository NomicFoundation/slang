// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    modifier m(uint256) {
        _;
    }

    function f(bool b) public {
        // Both branches name the modifier, so both are reported.
        (b ? m : m)(1);
    }
}
