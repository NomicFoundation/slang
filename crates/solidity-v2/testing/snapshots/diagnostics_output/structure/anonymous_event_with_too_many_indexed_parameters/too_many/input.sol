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
}
