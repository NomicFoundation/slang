// SPDX-License-Identifier: MIT
pragma solidity *;

// Base constructor arguments written as a constructor modifier run when
// the deriving contract is created.

contract A {
    constructor(uint256 x) {}
}

contract C is A {
    constructor() A(f()) {}
}

function f() returns (uint256) {
    new B();
    return 0;
}

contract B {
    constructor() {
        new C();
    }
}
