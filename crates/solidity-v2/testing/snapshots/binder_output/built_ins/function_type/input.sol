contract Test {
    function test() public {
        bytes4 v1 = this.test.selector;
        address v2 = this.test.address;

        bytes4 v3 = Foo.bar.selector;

        Foo f1;
        bytes4 v4 = f1.bar.selector;
        address v5 = f1.bar.address;

        bytes4 v6 = Baz.quux.selector;
    }
}

interface Foo {
    function bar() external payable;
}

library Baz {
    function quux() public {}
}
