contract Test {
    event Foo(uint x, uint y);
    event Foo(bool flag);

    function test() internal {
        emit Foo(1, 2);
        emit Foo(false);

        emit Foo({ x: 1, y: 2 });
        emit Foo({ flag: true });
    }
}
