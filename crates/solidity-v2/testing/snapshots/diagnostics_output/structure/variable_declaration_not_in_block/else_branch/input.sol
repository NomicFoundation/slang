// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint x) public pure {
        // A variable declaration cannot be the un-braced body of an `else`.
        if (x > 0) {
            uint a = 1;
        } else
            uint b = 2;

        // Wrapping the declaration in a block is fine.
        if (x > 0) {
            uint c = 3;
        } else {
            uint ok = 4;
        }
    }
}
