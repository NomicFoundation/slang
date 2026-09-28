// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported once: a type name written to is not a value, so it is not judged
// as a write target on top of that.

contract Test {
  function f() public pure {
    uint++;
  }
}
