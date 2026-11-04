// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {}

contract C is B {
    constructor() B {}
}
