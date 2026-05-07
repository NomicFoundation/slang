contract Foo {
    function bar(uint z) returns (uint) {
        return z + 1;
    }

    function baz(int x, int y) returns (int) {
        return x + y;
    }

    function quux(int x, int y) returns (int z) {
        z = x + y;
    }
}
