// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: each component of a tuple on the left hand side is written to, so
// one in calldata is rejected where it stands.

contract Test {
  function f(uint[] calldata items) external pure {
    uint other;
    (items[0], other) = (1, 2);
  }
}
