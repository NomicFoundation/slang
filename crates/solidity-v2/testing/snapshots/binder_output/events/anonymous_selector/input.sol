library Utils {
    event Foo(uint x) anonymous;

    function test() internal pure {
        Foo.selector;
    }
}
