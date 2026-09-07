pragma solidity *;

contract A {
    function foo() public virtual {}
}
contract B is A {
    function foo() public virtual override {
        super.foo();
    }
}
contract C is A {
    function foo() public virtual override {
        super.foo();
    }
}
contract D is B, C {
    function foo() public override(B, C) {
        super.foo();
    }
}
