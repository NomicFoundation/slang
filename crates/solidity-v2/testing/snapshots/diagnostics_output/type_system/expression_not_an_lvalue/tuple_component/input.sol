// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: every component of a tuple on the left hand side is written to.

contract Test {
  function f() internal pure returns (uint) {
    return 1;
  }

  function g() internal pure {
    uint value;
    (value, f()) = (1, 2);
  }
}
