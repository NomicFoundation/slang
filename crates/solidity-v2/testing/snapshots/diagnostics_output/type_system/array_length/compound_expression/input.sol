// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Neither an inline array nor a tuple is a foldable integer length.
    uint256[[2]] a;
    uint256[(1, 2)] b;
}
