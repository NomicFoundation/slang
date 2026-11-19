// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 x, uint256[] memory a) public pure {
        // An integer is not an array, a mapping or a type name, so it cannot be indexed.
        x[0];

        // Indexing an array is fine.
        a[0];
    }
}
