// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported on each component: a tuple is a value only if all of its
// components are.

contract Test {
  enum E { A }

  function f() public pure returns (E, E) {
    return (E, E);
  }
}
