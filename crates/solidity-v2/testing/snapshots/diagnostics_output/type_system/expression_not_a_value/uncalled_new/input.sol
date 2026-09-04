// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: `new Other` is not a value until it is called.

contract Other {}

contract Test {
  function f() public {
    Other flagged = new Other;
  }
}
