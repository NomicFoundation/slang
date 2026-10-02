// SPDX-License-Identifier: MIT
pragma solidity *;

// From slot 1, the array reaches `2**256`.
contract C layout at 1 {
    uint256[2 ** 256 - 1] x;
}
