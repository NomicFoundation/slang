// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a `public` constant is no more writable for having a getter.

contract Test {
  uint public constant LIMIT = 1;

  function f() internal pure {
    LIMIT = 2;
  }
}
