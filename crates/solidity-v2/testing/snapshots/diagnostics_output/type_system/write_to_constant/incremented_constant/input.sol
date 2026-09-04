// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: `++` writes back to its operand.

contract Test {
  uint constant LIMIT = 1;

  function f() internal pure {
    LIMIT++;
  }
}
