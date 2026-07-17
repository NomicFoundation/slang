contract Foo {
    function bar(uint z) public returns (uint) {
        return z + 1;
    }

    function baz(int x, int y) public returns (int) {
        return x + y;
    }

    function quux(int x, int y) public returns (int z) {
        z = x + y;
    }
}
