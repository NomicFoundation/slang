// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Exactly 4 indexed parameters is the maximum allowed.
    event AtLimit(
        address indexed a,
        address indexed b,
        uint256 indexed c,
        uint256 indexed d
    ) anonymous;
}
