// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: `delete` writes to its operand, which has to be a location.

contract Test {
  function f() internal {
    delete f;
  }
}
