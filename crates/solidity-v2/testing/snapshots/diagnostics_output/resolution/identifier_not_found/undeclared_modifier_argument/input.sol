// SPDX-License-Identifier: MIT
pragma solidity *;

// The body's locals are not visible to the modifier arguments.
contract C {
    modifier m(uint256 a) {
        _;
    }

    function f() public pure m(y) {
        uint256 y;
        y;
    }
}
