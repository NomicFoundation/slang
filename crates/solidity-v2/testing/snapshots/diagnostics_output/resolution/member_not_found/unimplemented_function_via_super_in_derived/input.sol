// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function f() public virtual;
}

contract B is A {
    function f() public virtual override {}
}

contract C is A, B {
    function f() public override(A, B) {
        // `super` starts at `B`, whose `f` is implemented.
        super.f();
    }
}
