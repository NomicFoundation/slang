// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes calldata b) external pure {
        // Indexing a slice of 'bytes' needs an index expression.
        b[1:][];
    }
}
