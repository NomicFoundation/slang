// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: `abi` as a statement expression is inert.

contract Test {
  function f() public view {
    abi;
  }
}
