// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: `++` writes back to its operand, which has to be a location.

contract Test {
  function f() internal pure returns (uint) {
    return 1;
  }

  function g() internal pure {
    f()++;
  }
}
