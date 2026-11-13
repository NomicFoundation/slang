// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract B {
    constructor(uint256 x) {}
}

abstract contract C is B {
    constructor() B {}
}
