// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // An anonymous event has no selector topic, so all 4 log topics are
    // available for indexed parameters, but no more than that.
    event TooMany(
        address indexed a,
        address indexed b,
        address indexed c,
        uint256 indexed d,
        uint256 indexed e
    ) anonymous;

    // Control: exactly 4 indexed parameters is the maximum allowed.
    event AtLimit(
        address indexed a,
        address indexed b,
        uint256 indexed c,
        uint256 indexed d
    ) anonymous;

    // Control: non-indexed parameters don't count towards the limit.
    event Mixed(
        address indexed a,
        address indexed b,
        uint256 indexed c,
        uint256 indexed d,
        string e
    ) anonymous;
}
