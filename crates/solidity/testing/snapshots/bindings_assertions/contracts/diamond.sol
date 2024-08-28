contract A {
    function foo() public virtual {
        //   ^def:1
    }
}

contract B is A {
    function foo() public virtual override {
        //   ^def:2
    }
}

contract C is A {
}

contract D is B, C {
    function foo() public virtual override {
        super.foo();
        //    ^ref:2
    }
}
