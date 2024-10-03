contract Test {
    function foo() public {}

    function bar() public returns (int) {
        this.foo();
    }
}
