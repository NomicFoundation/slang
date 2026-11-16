// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256[][] calldata a, uint256[2][] calldata b) public pure {
        // The elements of 'uint256[][]' are dynamically encoded, so it cannot
        // be sliced.
        a[1:];

        // Fixed-size elements of a static type are fine.
        b[1:];
    }
}
