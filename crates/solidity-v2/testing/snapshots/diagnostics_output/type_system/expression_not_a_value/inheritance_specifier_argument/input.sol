// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: the arguments of a base contract are its constructor's, so they
// are values.

contract Base {
  constructor(uint value) {}
}

contract Test is Base(abi) {}
