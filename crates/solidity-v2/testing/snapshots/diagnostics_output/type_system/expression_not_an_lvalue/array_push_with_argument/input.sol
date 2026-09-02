// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: `push(value)` writes the element itself and yields nothing to
// write to, unlike the no-argument form.

contract Test {
  uint[] items;

  function f() internal {
    items.push(1) = 2;
  }
}
