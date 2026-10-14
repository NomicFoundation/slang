// SPDX-License-Identifier: MIT
pragma solidity *;

// A zero base slot lays out as if none was given, so the contract itself is
// reported.
contract C layout at 0 {
    uint256[2 ** 255] a;
    uint256[2 ** 255] b;
}
