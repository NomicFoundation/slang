// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint8 constant B = 255;

    // `B + 1` folds to `256`, which does not fit the common type `uint8` of
    // the operands, so the addition is an arithmetic error.
    uint256[B + 1] x;
}
