// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256 constant A = B;
    uint256 constant B = A;
    uint256[A] data;
}
