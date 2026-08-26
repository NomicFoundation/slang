// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a condition is a value position, and `block` is a namespace.

contract Test {
  function f() public view {
    if (block) {}
  }
}
