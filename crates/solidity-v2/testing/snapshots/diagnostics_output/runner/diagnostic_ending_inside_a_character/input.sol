// SPDX-License-Identifier: MIT
pragma solidity *;

// solc reports byte offsets, and points at the first byte of the character it
// rejects here only, so its range ends inside that character.

contract C {
    uint é;
}
