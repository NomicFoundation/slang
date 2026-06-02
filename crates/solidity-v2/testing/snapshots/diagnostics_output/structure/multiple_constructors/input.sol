// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    constructor() {}
    constructor(uint256 _x) {}
}

// A contract with a single constructor is valid and must not be flagged.
contract D {
    constructor() {}
}
