// SPDX-License-Identifier: MIT
pragma solidity *;

// `2**9999` folds to an integer that needs more than 256 bits.

function pick(bool c) pure returns (uint256) {
    return c ? 2**9999 : 1;
}
