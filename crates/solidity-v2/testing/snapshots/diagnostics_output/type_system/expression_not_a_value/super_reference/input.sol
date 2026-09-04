// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: `super` has no value by itself.

contract Test {
  function f() public pure {
    uint flagged = super;
  }
}
