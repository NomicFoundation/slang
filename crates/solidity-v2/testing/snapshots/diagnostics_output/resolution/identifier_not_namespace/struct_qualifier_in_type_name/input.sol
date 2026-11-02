// SPDX-License-Identifier: MIT
pragma solidity *;

// A struct has fields, but no members to name in a type path.
struct S {
    uint256 x;
}

contract C {
    S.x v;
}
