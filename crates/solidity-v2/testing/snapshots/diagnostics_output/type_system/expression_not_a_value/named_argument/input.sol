// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a named argument's value is a value position, even where the
// callee is a single declaration that no overload set has to select from.

contract Test {
  function take(uint value) internal pure {}

  function f() public view {
    take({value: abi});
  }
}
