// SPDX-License-Identifier: MIT
pragma solidity *;

// The variables don't fit even from slot 0, so the contract itself is
// reported rather than its base slot.
contract C layout at 1 {
    uint256[2 ** 255] a;
    uint256[2 ** 255] b;
}
