contract A {
    function foo() public virtual {
        //   ^def:1
    }
}
contract B is A {
    function bar() public {
        foo();
        //<ref:1  -- this is fine when B is being compiled
    }
}
contract C is B {
    function foo() public override {
        //   ^def:2
    }
    function test() public {
        foo();
        //<ref:2
    }
}
