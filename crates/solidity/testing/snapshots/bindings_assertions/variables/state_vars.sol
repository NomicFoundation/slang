contract Foo {
    uint x;
    //   ^def:1

    function bar() returns (uint) {
        return x;
        //     ^ref:1
    }

    function baz() returns (int) {
        return y;
        //     ^ref:!
    }
}

contract Bar {
    function quux() returns (uint) {
        return x;
        //     ^ref:!
    }
}
