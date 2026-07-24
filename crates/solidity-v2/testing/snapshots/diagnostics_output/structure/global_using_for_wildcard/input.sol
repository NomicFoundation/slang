// SPDX-License-Identifier: MIT
pragma solidity *;

library L {}

struct S {
    uint256 x;
}

// A `global` directive can only attach functions to a specific type, not `*`.
using L for * global;

// Control: a `global` directive naming a user-defined type is allowed.
using L for S global;
