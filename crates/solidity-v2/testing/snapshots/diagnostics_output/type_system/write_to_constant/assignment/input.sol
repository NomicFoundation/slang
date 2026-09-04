// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a constant is fixed at compile time, so there is nothing to write
// to.

contract Test {
  uint constant LIMIT = 1;

  function f() internal pure {
    LIMIT = 2;
  }
}
