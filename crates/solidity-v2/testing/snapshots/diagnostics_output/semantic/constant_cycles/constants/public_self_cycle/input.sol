// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A public constant keeps its state variable shape in the IR, but still
    // takes part in constant cycle detection.
    uint256 public constant A = A;
}
