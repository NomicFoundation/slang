// SPDX-License-Identifier: MIT
pragma solidity *;

// An event's parameters are not visible to the types of its parameters, so `S`
// still names the struct.
struct S {
    uint256 a;
}

contract C {
    event E(uint256 S, S s);
}
