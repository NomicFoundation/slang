// SPDX-License-Identifier: MIT
pragma solidity *;

// In B's bytecode, `super.f()` runs A's implementation, which creates B.

contract A {
    function f() public virtual {
        new B();
    }
}

contract B is A {
    function f() public override {
        super.f();
    }
}
