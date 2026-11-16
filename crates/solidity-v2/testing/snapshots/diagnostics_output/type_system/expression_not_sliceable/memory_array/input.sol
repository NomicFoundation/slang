// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256[] memory a, uint256[] calldata b) public pure {
        // Only arrays in calldata can be sliced.
        a[1:];

        // Slicing a dynamically-sized calldata array is fine.
        b[1:];
    }
}
