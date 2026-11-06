// SPDX-License-Identifier: MIT
pragma solidity *;

// A constructor invokes a contract that is not one of its bases.
contract A {}

contract C {
    constructor() A() {}
}
