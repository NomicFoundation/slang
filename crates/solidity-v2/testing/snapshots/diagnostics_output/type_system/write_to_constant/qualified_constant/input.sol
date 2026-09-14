// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a constant reached through the name of its contract.

contract Other {
  uint internal constant LIMIT = 1;
}

contract Test is Other {
  function f() internal pure {
    Other.LIMIT = 2;
  }
}
