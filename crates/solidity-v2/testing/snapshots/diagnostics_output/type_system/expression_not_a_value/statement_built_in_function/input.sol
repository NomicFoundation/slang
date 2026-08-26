// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: `keccak256` as a statement expression is inert.

contract Test {
  function f() public pure {
    keccak256;
  }
}
