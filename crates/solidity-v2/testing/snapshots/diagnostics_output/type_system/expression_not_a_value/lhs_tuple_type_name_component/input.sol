// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported once: a component of a tuple on the left hand side that names a
// type is not a value, so it is not judged as a write target either.

contract Test {
  enum E { A }

  function f() public pure {
    uint x;
    (x, E) = (1, E.A);
  }
}
