// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: an event's arguments are values, positional or named.

contract Test {
  event Flagged(uint value);

  function f() public {
    emit Flagged(abi);
  }

  function g() public {
    emit Flagged({value: tx});
  }
}
