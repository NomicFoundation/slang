// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: calldata is the read-only input of a call, so an element of a
// calldata array cannot be written to, nor a byte of calldata `bytes`.

contract Test {
  function f(uint[] calldata items, bytes calldata data) external pure {
    items[0] = 1;
    data[0] = 0x01;
    delete items[1];
  }
}
