// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(string memory s) public pure {
        // Indexing a 'string' needs an index expression. The missing index is
        // reported rather than the 'string' itself not being indexable.
        s[];
    }
}
