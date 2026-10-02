// SPDX-License-Identifier: MIT
pragma solidity *;

// Both values share the last slot, which still reaches `2**256`.
contract C layout at 2 ** 256 - 1 {
    uint8 x;
    uint8 y;
}
