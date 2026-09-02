// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a built-in member of a type is not a location of its own.

contract Test {
  function f() internal pure {
    address(0).balance = 7;
  }
}
