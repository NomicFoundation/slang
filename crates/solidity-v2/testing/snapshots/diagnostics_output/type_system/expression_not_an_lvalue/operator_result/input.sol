// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: the result of an operator is a value, not a location.

contract Test {
  function f() internal pure {
    uint left;
    uint right;
    left + right = 3;
  }
}
