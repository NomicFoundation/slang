// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a constant is read like any other value.

contract Test {
  uint constant LIMIT = 1;

  function f() internal pure returns (uint) {
    uint value = LIMIT;
    return value + LIMIT;
  }
}
