// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: only a plain assignment destructures a tuple.

contract Test {
  function f() internal pure {
    uint a;
    uint b;
    (a, b) += (1, 2);
    (((a, b))) += (1, 2);
  }
}
