// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: an argument of an ordinary call is a value position, and a
// user-defined type name names a type.

contract Test {
  enum E { A }

  function take(uint) internal pure {}

  function f() public pure {
    take(E);
  }
}
