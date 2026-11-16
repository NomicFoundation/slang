// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes4 b) public pure {
        // Indexing a fixed-size 'bytesN' needs an index expression.
        b[];
    }
}
