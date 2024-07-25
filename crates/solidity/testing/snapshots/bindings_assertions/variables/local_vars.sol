contract Foo {
    function bar() returns (uint) {
        uint x = 10;
        //   ^def:1

        return x + 2;
        //     ^ref:1
    }

    function baz() returns (int) {
        return w + x;
        //         ^ref:!
        //     ^ref:!
    }
}
