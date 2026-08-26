// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: `_` is the modifier body placeholder, a statement not a value.

contract Test {
  modifier m() {
    _;
  }

  function f() public m {}
}
