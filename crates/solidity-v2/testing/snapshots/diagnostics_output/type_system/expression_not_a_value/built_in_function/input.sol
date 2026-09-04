// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a built-in function has no value of its own to assign.

contract Test {
  function f() public pure {
    bytes32 flagged = keccak256;
  }
}
