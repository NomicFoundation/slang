// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // The cycle crosses both constant shapes. The edge out of A targets a
    // public constant, and the edge out of B targets a non-public one.
    uint256 internal constant A = B;
    uint256 public constant B = A;
}
