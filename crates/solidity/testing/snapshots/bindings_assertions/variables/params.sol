contract Foo {
    function bar(uint z) returns (uint) {
        //            ^def:1

        return z + 1;
        //     ^ref:1
    }

    function baz() returns (int) {
        return z + 2;
        //     ^ref:!
    }

    function quux(int x) returns (int y) {
        //                            ^def:2
        //            ^def:3
        y = x + 5;
        //  ^ref:3
    //  ^ref:2
    }
}
