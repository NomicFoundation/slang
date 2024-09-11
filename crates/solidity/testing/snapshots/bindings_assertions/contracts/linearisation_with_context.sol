// --- context: D
contract A {
    function foo() public {}
    //       ^def:1
}
contract B is A {
    function foo() public {
        //   ^def:2
        super.foo();
        //    ^ref:1
    }
}
contract C is A {
    function foo() public {
        //   ^def:3
        super.foo();
        //    ^ref:2
    }
}
contract D is B, C {
    function foo() public {
        //   ^def:4
        super.foo();
        //    ^ref:3
    }
}
