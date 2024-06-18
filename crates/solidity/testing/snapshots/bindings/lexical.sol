contract Foo {
    uint y;
    //   ^def:1

    function bar(uint z) returns (uint) {
        //            ^def:2
        uint x = 10;
        //   ^def:3

        return x + y + z;
        //             ^ref:2
        //         ^ref:1
        //     ^ref:3
    }

    function baz() returns (int) {
        return w + z + x;
        //             ^ref:!
        //         ^ref:!
        //     ^ref:!
    }
}
