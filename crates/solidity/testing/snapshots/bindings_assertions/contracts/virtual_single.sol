contract A {
    function foo() public virtual {
        //   ^def:1
    }
}
contract B is A {
    function bar() public {
        foo();
        //<ref:1  -- this is fine when B is being compiled
        //<ref:2 (=0.0.0)  -- this should be the result when compiling C
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
