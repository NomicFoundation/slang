contract Base {
    struct Data {
        uint x;
    }
    function foo(Data[] memory d) virtual external {}
    function bar(Data[] calldata d) virtual external {}
    function foo2(Data[] memory d) virtual external {}
    function bar2(Data[] calldata d) virtual external {}
}

contract Foo is Base {
    // All these changes in memory location are valid because the base function
    // is external.
    function foo(Data[] calldata d) override external {}
    function bar(Data[] memory d) override external {}
    function foo2(Data[] calldata d) override public {}
    function bar2(Data[] memory d) override public {}

    function test() internal view {
        this.foo;
        this.bar;
        this.foo2;
        this.bar2;
    }
}
