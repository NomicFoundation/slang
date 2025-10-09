contract Test {
    event Foo(uint count);

    function test() internal {
        Foo({count: 1});
    }
}
