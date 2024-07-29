contract Sample {
    function test() returns (int) {
        return from_sample() + top_level() + MyLib.from_lib();
        //                                         ^ref:3
        //                                   ^ref:4
        //                     ^ref:! (<0.7.1)
        //                     ^ref:1 (>=0.7.1)
        //     ^ref:2
    }
    function from_sample() returns (int) {
        //   ^def:2
        return 1;
    }
}

library MyLib {
    //  ^def:4
    function from_lib() returns (int) {
        //   ^def:3
        return 3;
    }
}

function top_level() returns (int) {
    //   ^def:1
    return 2;
}
