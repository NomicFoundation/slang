library Utils {
    enum Answer { Yes, No }
    //   ^def:1

    function runCallback(Answer input, function (Answer) f) internal {
        //                                               ^def:2
        //                                       ^ref:1
        f(input);
        //<ref:2
    }

    function map(Answer input, function (Answer) returns (Answer) f)
        //                                                        ^def:3
        //                                                ^ref:1
        //                               ^ref:1
        internal returns (Answer result)
    {
        result = f(input);
        //       ^ref:3
    }
}
