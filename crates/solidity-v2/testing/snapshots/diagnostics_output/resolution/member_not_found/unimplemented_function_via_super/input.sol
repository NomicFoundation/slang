// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function f() public virtual;
}

contract B is A {
    function f() public override {
        super.f();
    }
}
