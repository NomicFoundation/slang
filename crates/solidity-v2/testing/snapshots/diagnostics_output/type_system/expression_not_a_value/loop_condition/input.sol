// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a loop's condition is a value position.
//
// A `for` loop's condition is written as an expression statement, so it is not
// checked yet and is left out here. See the TODO in `leave_for_statement`.

contract Test {
  function f() public view {
    while (abi) {}
    do {} while (tx);
  }
}
