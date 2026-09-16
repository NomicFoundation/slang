// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a component of a tuple on the left hand side is a value position
// like any other, and it is reported once, by the tuple.

contract Test {
  function f() public pure {
    uint right;
    (abi, right) = (1, 2);
  }
}
