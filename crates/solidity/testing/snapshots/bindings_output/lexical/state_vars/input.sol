contract Foo {
    uint x;

    function bar() returns (uint) {
        return x;
    }

    function baz() returns (int) {
        return y;
    }
}

contract Bar {
    function quux() returns (uint) {
        return x;
    }
}
