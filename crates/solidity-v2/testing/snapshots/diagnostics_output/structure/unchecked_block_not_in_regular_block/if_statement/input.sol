// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint x) public pure {
        // An `unchecked` block cannot be the un-braced body of an `if`.
        if (x > 0)
            unchecked {}

        // Nested directly inside a regular block is fine.
        if (x > 0) {
            unchecked {}
        }
    }
}
