// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a conditional expression yields a value, even where both branches
// are locations.

contract Test {
  function f(bool condition) internal pure {
    uint left;
    uint right;
    (condition ? left : right) = 1;
  }
}
