// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint x) public pure {
        // A variable declaration cannot be the un-braced body of an `if`.
        if (x > 0)
            uint a = 1;

        // Wrapping the declaration in a block is fine.
        if (x > 0) {
            uint ok = 2;
        }
    }
}
