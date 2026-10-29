// SPDX-License-Identifier: MIT
pragma solidity *;

// `a` packs 32 elements per slot, so `b` still fits after it.
contract C {
    uint8[2 ** 256 - 1] a;
    uint8 b;
}
