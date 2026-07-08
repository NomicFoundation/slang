// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function foo() public virtual;
    function foo(uint x) public virtual returns (uint);
    modifier mod() virtual;
}

// `B` leaves `foo()` unimplemented.
contract B is A {
    function foo(uint x) public override returns (uint) { return x; }
    modifier mod() override { _; }
}

// `C` leaves `foo(uint)` unimplemented.
contract C is A {
    function foo() public override {}
    modifier mod() override { _; }
}

// `D` leaves `mod()` unimplemented.
contract D is A {
    function foo() public override {}
    function foo(uint x) public override returns (uint) { return x; }
}

// `E` implements every inherited member, so it need not be `abstract`.
contract E is A {
    function foo() public override {}
    function foo(uint x) public override returns (uint) { return x; }
    modifier mod() override { _; }
}
