// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a constant declared at file level is written to the same way.

uint constant LIMIT = 1;

contract Test {
  function f() internal pure {
    LIMIT = 2;
  }
}
