// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(string memory s) public pure {
        // A 'string' cannot be indexed, even without an index expression.
        s[];
    }
}
