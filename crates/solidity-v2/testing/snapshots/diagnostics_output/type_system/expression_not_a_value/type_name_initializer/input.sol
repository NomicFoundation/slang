// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: an initialiser is a value position, and an elementary type keyword
// names a type.

contract Test {
  function f() public pure {
    uint256 x = uint;
    x;
  }
}
