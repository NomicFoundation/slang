// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        // Invalid: a parenthesised empty tuple `()` is an empty assignment
        // target. The right hand side is a legal empty-tuple value, so no
        // further error follows.
        (()) = ();
    }
}
