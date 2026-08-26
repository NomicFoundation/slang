// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: an operand is a value position, and `tx` is a namespace.

contract Test {
  function f() public view {
    uint flagged = 1 + tx;
  }
}
