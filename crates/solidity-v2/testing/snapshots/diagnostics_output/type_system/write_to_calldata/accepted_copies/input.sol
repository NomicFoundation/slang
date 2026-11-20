// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a calldata variable can be pointed at other calldata, and a copy
// of calldata in memory or in storage is written to freely.

contract Test {
  struct Pair {
    uint left;
    uint right;
  }

  uint[] stored;

  function f(uint[] calldata items, uint[] calldata others, Pair calldata pair) external {
    items = others;
    uint[] memory copy = items;
    copy[0] = 1;
    Pair memory local = pair;
    local.left = 2;
    stored = items;
    stored[0] = 3;
  }
}
