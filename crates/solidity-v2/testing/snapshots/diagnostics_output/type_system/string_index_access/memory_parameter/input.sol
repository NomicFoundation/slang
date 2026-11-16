// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(string memory s) public pure {
        // A 'string' has no element type, so it cannot be indexed.
        s[0];
    }

    // Indexing `bytes` is fine.
    function g(bytes memory b) public pure returns (bytes1) {
        return b[0];
    }
}
