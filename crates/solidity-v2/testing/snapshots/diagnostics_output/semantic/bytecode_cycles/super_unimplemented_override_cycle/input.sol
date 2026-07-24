// SPDX-License-Identifier: MIT
pragma solidity *;

// C.f has no body, so `super.f()` in D runs B.f, which creates D.

abstract contract B {
    function f() public virtual {
        new D();
    }
}

abstract contract C {
    function f() public virtual;
}

contract D is B, C {
    function f() public override(B, C) {
        super.f();
    }
}
