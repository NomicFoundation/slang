contract CustomArrays {
    uint constant SIZE = 5;
    //            ^def:7

    struct Value {
        // ^def:1
        uint value;
        //   ^def:2
    }

    Value[][SIZE] values;
    //            ^def:3
    //      ^ref:7
    //<ref:1

    function test() public returns (uint total) {
        //                               ^def:6
        for (uint i = 0; i < values.length; i++) {
            //               ^ref:3
            //    ^def:4
            for (uint j = 0; j < SIZE - 1; j++) {
                //    ^def:5
                values[i][j + 1].value += values[i][j].value;
                //                                     ^ref:2
                //                                  ^ref:5
                //                               ^ref:4
                //                        ^ref:3
                //               ^ref:2
                //        ^ref:5
                //     ^ref:4
                //<ref:3
            }
            total += values[i][SIZE - 1].value;
            //                           ^ref:2
            //                 ^ref:7
            //              ^ref:4
            //       ^ref:3
            //<ref:6
        }
    }
}
