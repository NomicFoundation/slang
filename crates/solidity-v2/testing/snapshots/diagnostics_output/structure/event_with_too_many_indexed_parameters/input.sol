// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A non-anonymous event spends its first log topic on the event selector,
    // leaving room for only 3 indexed parameters.
    event TooMany(address indexed a, address indexed b, address indexed c, uint256 indexed d);

    // Control: exactly 3 indexed parameters is the maximum allowed.
    event AtLimit(address indexed a, address indexed b, uint256 indexed c);

    // Control: non-indexed parameters don't count towards the limit.
    event Mixed(address indexed a, address indexed b, uint256 indexed c, uint256 d, string e);
}
