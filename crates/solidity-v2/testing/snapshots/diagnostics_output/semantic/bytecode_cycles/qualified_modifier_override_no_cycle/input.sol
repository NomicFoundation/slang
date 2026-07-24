// SPDX-License-Identifier: MIT
pragma solidity *;

// The qualified invocation runs A.m, so the overriding modifier that
// creates B never runs and no bytecode cycle exists.

contract A {
    modifier m() virtual {
        _;
    }
}

contract C is A {
    modifier m() override {
        new B();
        _;
    }

    function f() public A.m {}
}

contract B {
    constructor() {
        new C();
    }
}
