// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint x) public pure {
        // A variable declaration cannot be the un-braced body of a `do`-`while` loop.
        do
            uint a = 1;
        while (x > 0);

        // Wrapping the declaration in a block is fine.
        do {
            uint ok = 2;
        } while (x > 0);
    }
}
