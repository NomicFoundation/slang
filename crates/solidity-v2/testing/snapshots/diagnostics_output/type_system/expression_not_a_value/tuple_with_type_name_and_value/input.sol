// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported on the component that names a type.

contract Test {
  function f() public pure {
    (uint a, uint b) = (uint, 1);
    a;
    b;
  }
}
