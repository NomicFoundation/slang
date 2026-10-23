// SPDX-License-Identifier: MIT
pragma solidity *;

contract D {
    uint256 constant X = 10;
}

contract C layout at D.X {}
