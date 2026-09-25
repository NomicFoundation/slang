// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported on a component that names a type however deeply it is nested.

contract Test {
  function f() public pure {
    uint x;
    uint y;
    uint z;
    (x, (y, z)) = (1, (2, uint));
  }
}
