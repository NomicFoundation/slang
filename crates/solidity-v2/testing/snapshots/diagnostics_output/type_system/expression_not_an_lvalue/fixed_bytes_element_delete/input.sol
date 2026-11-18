// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: deleting a single byte of a fixed-size byte array writes to it, and
// the byte is copied out of the word holding it, so it is not a location.

contract C {
  function f() public pure {
    bytes16 b;
    delete b[5];
  }
}
