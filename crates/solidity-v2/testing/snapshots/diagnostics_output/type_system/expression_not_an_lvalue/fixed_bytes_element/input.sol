// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: indexing a fixed-size byte array copies the byte out, so the
// result is not a location.

contract Test {
  function f() internal pure {
    bytes32 word;
    word[0] = 0x01;
  }
}
