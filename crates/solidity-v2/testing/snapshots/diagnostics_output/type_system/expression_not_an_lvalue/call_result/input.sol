// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: the result of a call is a value, and parentheses do not make it a
// location.

contract Test {
  function f() internal pure returns (uint) {
    return 1;
  }

  function g() internal pure {
    (f()) = 2;
  }
}
