// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a variable of function type is a location, and a function name
// denotes a value that can be assigned to it.

contract Test {
  function f() internal pure returns (int) {
    return 400;
  }

  function test() internal pure {
    function () internal pure returns (int) g;
    g = f;
  }
}
