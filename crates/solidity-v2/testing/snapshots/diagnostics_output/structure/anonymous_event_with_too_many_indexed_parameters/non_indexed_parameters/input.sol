// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Non-indexed parameters don't count towards the limit.
    event Mixed(
        address indexed a,
        address indexed b,
        uint256 indexed c,
        uint256 indexed d,
        string e
    ) anonymous;
}
