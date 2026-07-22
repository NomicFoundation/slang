// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint x) public pure {
        // An `unchecked` block cannot be the un-braced body of a `for` loop.
        for (uint i = 0; i < x; ++i)
            unchecked {}

        // Nested directly inside a regular block is fine.
        for (uint i = 0; i < x; ++i) {
            unchecked {}
        }
    }
}
