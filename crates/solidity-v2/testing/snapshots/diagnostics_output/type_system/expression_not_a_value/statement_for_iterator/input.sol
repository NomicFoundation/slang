// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a `for` loop's iterator is a statement, so naming a built-in
// there is inert, just as in an expression statement.

contract Test {
  function f() public pure {
    for (uint i = 0; i < 3; abi) {
    }
  }
}
