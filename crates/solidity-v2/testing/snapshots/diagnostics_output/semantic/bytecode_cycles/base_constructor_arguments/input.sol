// SPDX-License-Identifier: MIT
pragma solidity *;

// Base constructor arguments run during B's creation, so the creation inside
// the free function belongs to B.

function f() returns (uint256) {
    new B();
    return 0;
}

contract A {
    constructor(uint256) {}
}

contract B is A(f()) {}
