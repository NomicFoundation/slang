// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: `++`, `--`, a compound assignment and `delete` all write to a
// location.

contract Test {
  uint[] items;

  function f() internal {
    uint value;
    value++;
    --value;
    value += 1;
    delete value;
    delete items[0];
    delete items;
  }
}
