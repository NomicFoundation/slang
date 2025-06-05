library Utils {
    event Foo(uint x);

    function test() internal pure {
        Foo.selector;
    }
}
