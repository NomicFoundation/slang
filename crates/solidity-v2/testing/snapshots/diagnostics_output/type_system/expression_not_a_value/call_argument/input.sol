// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: an argument is a value position, and `msg` is a namespace.

contract Test {
  function take(uint) internal pure {}

  function f() public view {
    take(msg);
  }
}
