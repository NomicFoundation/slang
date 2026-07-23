// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint x) public pure {
        // A variable declaration cannot be the un-braced body of a `while` loop.
        while (x > 0)
            uint a = 1;

        // Wrapping the declaration in a block is fine.
        while (x > 0) {
            uint ok = 2;
        }
    }
}
