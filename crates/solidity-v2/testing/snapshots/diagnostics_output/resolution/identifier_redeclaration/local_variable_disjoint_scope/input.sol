// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        uint x;
        {
            // Shadowing in a new lexical scope is legal.
            uint x;
        }
        // Redeclaring in the same scope is not, even after the inner block.
        uint x;
    }
}
