// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: an enum member names a constant of the enum, not a location.

contract Test {
  enum Choice { Left, Right }

  function f() internal pure {
    Choice.Left = Choice.Right;
  }
}
