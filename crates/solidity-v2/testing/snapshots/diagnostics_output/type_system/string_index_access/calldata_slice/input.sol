// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(string calldata s) public pure {
        // Slicing a calldata `string` is fine.
        s[1:];

        // A slice of a 'string' cannot be indexed either.
        s[1:][0];
    }
}
