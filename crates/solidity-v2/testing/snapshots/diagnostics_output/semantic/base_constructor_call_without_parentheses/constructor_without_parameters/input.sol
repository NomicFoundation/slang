// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {
    constructor() {}
}

contract C is B {
    constructor() B {}
}
