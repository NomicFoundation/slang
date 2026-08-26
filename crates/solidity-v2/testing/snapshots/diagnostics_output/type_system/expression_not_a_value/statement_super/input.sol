// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: `super` as a statement expression is inert.

contract Test {
  function f() public pure {
    super;
  }
}
