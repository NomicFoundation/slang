// SPDX-License-Identifier: MIT
pragma solidity *;

// Each element fits, but two of them span `2**256` slots.
contract C {
    uint256[2 ** 255][2] a;
}
