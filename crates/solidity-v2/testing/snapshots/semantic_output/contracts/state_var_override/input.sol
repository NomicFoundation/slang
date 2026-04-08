interface IFoo {
    function foo() external returns (uint, uint);
}

contract Test is IFoo {
    struct Bar {
        uint x;
        uint y;
    }
    Bar public override foo;

    function test() internal view {
        foo.x;
        this.foo;
        this.foo();
    }
}
