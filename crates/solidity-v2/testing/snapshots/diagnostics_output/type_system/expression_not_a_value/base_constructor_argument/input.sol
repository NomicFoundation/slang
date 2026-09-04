// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a base constructor's arguments are values, given here on the
// constructor rather than on the contract.

contract Base {
  constructor(uint value) {}
}

contract Test is Base {
  constructor() Base(abi) {}
}
