// SPDX-License-Identifier: MIT
pragma solidity *;

// From slot 1, `b` and `c` share the last slot, so the variables fit.
contract C layout at 1 {
    uint256[2 ** 256 - 3] a;
    uint8 b;
    uint8 c;
}
