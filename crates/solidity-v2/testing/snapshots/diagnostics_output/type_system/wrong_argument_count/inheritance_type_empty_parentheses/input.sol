// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {
    constructor(uint256 x) {}
}

abstract contract C is B() {}
