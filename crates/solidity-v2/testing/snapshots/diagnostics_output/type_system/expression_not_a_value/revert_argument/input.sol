// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: an error's arguments are values, positional or named.

error Flagged(uint value);

contract Test {
  function f() public view {
    revert Flagged(abi);
  }

  function g() public view {
    revert Flagged({value: tx});
  }
}
