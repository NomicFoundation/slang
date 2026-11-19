// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(string calldata s) external pure {
        // A slice of a 'string' cannot be indexed, even without an index
        // expression.
        s[1:][];
    }
}
