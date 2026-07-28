// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A public constant keeps its state variable shape in the IR, but is
    // still a compile-time constant usable as an array length.
    uint256 public constant LEN = 3;
    uint256[LEN] x;
}
