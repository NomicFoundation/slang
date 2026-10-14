// SPDX-License-Identifier: MIT
pragma solidity *;

// Each array fits, but together they span `2**256` slots.
contract C {
    uint256[2 ** 255] a;
    uint256[2 ** 255] b;
}
