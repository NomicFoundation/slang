// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a tuple nested in the left hand side is written component-wise
// too, so every component is judged however deeply it is nested.

contract Test {
  function value() internal pure returns (uint) {
    return 1;
  }

  function f() internal pure {
    uint a;
    uint b;
    (a, (b, value())) = (1, (2, 3));
  }
}
