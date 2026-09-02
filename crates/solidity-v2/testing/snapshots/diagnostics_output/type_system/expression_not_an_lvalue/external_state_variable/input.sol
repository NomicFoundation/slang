// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a state variable reached through a contract reference is read
// through its getter, and there is no setter to write through.

contract Test {
  uint public value;

  function f() external {
    delete this.value;
  }
}
