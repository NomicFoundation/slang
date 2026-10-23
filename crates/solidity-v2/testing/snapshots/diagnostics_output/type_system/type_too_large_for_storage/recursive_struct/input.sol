// SPDX-License-Identifier: MIT
pragma solidity *;

// A struct can refer to itself behind a dynamic array or a mapping, and
// storing it is fine.

struct Node {
    uint256 value;
    Node[] children;
    mapping(uint256 => Node) byKey;
}

contract C {
    Node root;
}
