// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A non-anonymous event spends its first log topic on the event selector,
    // leaving room for only 3 indexed parameters.
    event TooMany(address indexed a, address indexed b, address indexed c, uint256 indexed d);
}
