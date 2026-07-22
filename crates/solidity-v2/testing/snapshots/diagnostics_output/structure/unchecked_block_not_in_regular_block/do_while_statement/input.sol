// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint x) public pure {
        // An `unchecked` block cannot be the un-braced body of a `do`-`while` loop.
        do
            unchecked {}
        while (x > 0);

        // Nested directly inside a regular block is fine.
        do {
            unchecked {}
        } while (x > 0);
    }
}
