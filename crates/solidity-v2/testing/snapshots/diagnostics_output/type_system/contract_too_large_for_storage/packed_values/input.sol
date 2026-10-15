// SPDX-License-Identifier: MIT
pragma solidity *;

// `b` and `c` share the last slot, so the variables fit in `2**256 - 1` slots.
contract C {
    uint256[2 ** 256 - 2] a;
    uint8 b;
    uint8 c;
}
