// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a range index yields a slice, which is a read-only view of the
// array it comes from.

contract Test {
  function f(uint[] calldata items) external pure {
    items[0:1] = items;
  }
}
