// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        // A string literal cannot be indexed.
        "abc"[0];
    }
}
