// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a pair of parentheses only groups, so the target has the type of
// the expression inside it rather than a tuple type.

contract Test {
  function f() internal pure {
    uint a;
    (a) += 2;
    ((a)) -= 1;
  }
}
