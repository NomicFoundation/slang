// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a modifier's arguments are values.

contract Test {
  modifier m(uint value) {
    _;
  }

  function f() public view m(abi) {}
}
