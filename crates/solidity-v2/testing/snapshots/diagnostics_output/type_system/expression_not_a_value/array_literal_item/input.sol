// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: every item of an array literal is a value position, and each one
// is reported rather than only the first.

contract Test {
  function f() public view {
    uint[2] memory flagged = [abi, tx];
  }
}
