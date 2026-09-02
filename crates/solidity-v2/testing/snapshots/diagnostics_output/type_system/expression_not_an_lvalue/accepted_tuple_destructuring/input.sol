// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a tuple of locations, and an omitted component, which writes
// nothing.

contract Test {
  function pair() internal pure returns (uint, uint) {
    return (1, 2);
  }

  function f() internal pure {
    uint left;
    uint right;
    (left, right) = pair();
    (left, ) = pair();
  }
}
