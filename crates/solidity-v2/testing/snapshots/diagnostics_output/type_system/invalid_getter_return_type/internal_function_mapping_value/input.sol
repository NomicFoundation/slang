// SPDX-License-Identifier: MIT
pragma solidity *;

// The getter returns the mapping value, an internal function.
contract C {
    mapping(uint256 => function() internal) public values;
}
