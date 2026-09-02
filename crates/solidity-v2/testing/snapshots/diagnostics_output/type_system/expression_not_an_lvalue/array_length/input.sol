// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: `length` is read-only, and an array cannot be resized through it.

contract Test {
  uint[] items;

  function f() internal {
    items.length = 5;
  }
}
