// SPDX-License-Identifier: MIT
pragma solidity *;

// In D's bytecode, `super.f()` inside C resolves along D's linearisation to
// B.f, which creates D. Resolving super along C's own linearisation would
// miss the cycle.

contract A {
    function f() public virtual {}
}

contract B is A {
    function f() public virtual override {
        new D();
    }
}

contract C is A {
    function f() public virtual override {
        super.f();
    }
}

contract D is B, C {
    function f() public override(B, C) {
        super.f();
    }
}
