// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: `delete` writes to its operand.

contract Test {
  uint constant LIMIT = 1;

  function f() internal pure {
    delete LIMIT;
  }
}
