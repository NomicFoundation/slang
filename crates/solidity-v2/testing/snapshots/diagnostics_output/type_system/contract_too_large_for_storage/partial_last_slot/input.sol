// SPDX-License-Identifier: MIT
pragma solidity *;

// `a` ends one slot short of the end, so `b` is laid out at the last slot which
// overflows the total number of slots occupied by the contract even if `b`
// itself does not need the full slot.
contract C {
    uint256[2 ** 256 - 1] a;
    uint8 b;
}
