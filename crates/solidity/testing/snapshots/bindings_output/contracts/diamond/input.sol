contract A {
    function foo() public virtual {
    }
}

contract B is A {
    function foo() public virtual override {
    }
}

contract C is A {
}

contract D is B, C {
    function foo() public virtual override {
        super.foo();
    }
}
