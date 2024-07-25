contract Foo {
    uint x = 10;
    //   ^def:1

    function a_func(uint x) returns (uint) {
        //               ^def:2
        uint y = x + 1;
        //       ^ref:2 (>=0.5.0)
        //   ^def:3
        {
            uint x = 20;
            //   ^def:4
            return x + y;
            //         ^ref:3
            //     ^ref:4
        }
    }

    function another_func() returns (uint y) {
        //                                ^def:5
        y = x + w;
     // ^ref:5
        //      ^ref:! (>=0.5.0)
        //      ^ref:6 (<0.5.0)
        //  ^ref:1
        uint w = 5;
        //   ^def:6
    }
}
