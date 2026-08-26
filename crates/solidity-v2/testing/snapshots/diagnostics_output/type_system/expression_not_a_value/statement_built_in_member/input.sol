// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: `abi.encode` as a statement expression is inert.

contract Test {
  function f() public pure {
    abi.encode;
  }
}
