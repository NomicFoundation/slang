// --- context: D
contract A {
    function foo() public {}
}
contract B is A {
    function foo() public {
        super.foo();
    }
}
contract C is A {
    function foo() public {
        super.foo();
    }
}
contract D is B, C {
    function foo() public {
        super.foo();
    }
}
