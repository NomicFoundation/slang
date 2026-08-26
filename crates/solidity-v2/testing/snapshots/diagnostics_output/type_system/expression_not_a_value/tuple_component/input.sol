// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: every component of a multi-component tuple is a value position.

contract Test {
  function f() public view {
    uint left;
    bool right;
    (left, right) = (abi, tx);
  }
}
