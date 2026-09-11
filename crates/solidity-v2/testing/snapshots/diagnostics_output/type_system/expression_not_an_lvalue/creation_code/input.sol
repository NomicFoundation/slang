// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: the creation code of a contract is a value the compiler produces.

contract Other {}

contract Test {
  function f() internal pure {
    type(Other).creationCode = new bytes(6);
  }
}
