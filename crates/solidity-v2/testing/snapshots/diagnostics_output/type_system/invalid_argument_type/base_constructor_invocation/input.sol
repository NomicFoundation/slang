// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {
    constructor(uint256 x) {}
}

contract C is B {
    constructor() B(true) {}
}
