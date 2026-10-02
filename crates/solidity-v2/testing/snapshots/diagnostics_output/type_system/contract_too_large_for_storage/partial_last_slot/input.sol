// SPDX-License-Identifier: MIT
pragma solidity *;

// `a` ends one slot short of the end, but `b` starts a slot that can never
// be completed.
contract C {
    uint256[2 ** 256 - 1] a;
    uint8 b;
}
