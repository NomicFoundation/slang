// SPDX-License-Identifier: MIT
pragma solidity *;

// A modifier invoked by qualified name runs the named contract's
// modifier, not its most derived override. A.m runs in C.f and creates B.

contract A {
    modifier m() virtual {
        new B();
        _;
    }
}

contract C is A {
    modifier m() override {
        _;
    }

    function f() public A.m {}
}

contract B {
    constructor() {
        new C();
    }
}
