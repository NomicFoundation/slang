contract A {
    function foo() public virtual {
        //   ^def:1
    }
}
contract B is A {
    function bar() public {
        foo();
        //<ref:1
    }
}
contract C is B {
    function foo() public override {
        //   ^def:2
    }
}
