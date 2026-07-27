// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        uint256 a;
        uint256 b;
        // Invalid: the right hand side of an assignment is a read position, so
        // a missing component there is not allowed. The left hand side (an
        // l-value) may legally omit slots and is not flagged.
        (a, ) = (b, );
    }
}
