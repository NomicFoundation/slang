// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    constructor(uint256) {}
}

contract B is A(x) {}
