// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint x) public pure {
        // A variable declaration cannot be the un-braced body of a `for` loop.
        for (uint i = 0; i < x; ++i)
            uint a = 1;

        // Wrapping the declaration in a block is fine.
        for (uint i = 0; i < x; ++i) {
            uint ok = 2;
        }
    }
}
