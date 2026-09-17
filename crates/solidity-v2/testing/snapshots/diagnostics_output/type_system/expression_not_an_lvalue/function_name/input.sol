// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a function name denotes no location, so it cannot be assigned to.

contract Test {
  function f() internal {}

  function g() internal {
    g = f;
  }
}
