contract Test {
    function with_tuple_decon() public {
        int a = 1;
        //  ^def:1
        for ((int i, int b) = (0, a); i < 10; ) {
            //    ^def:2
            //           ^def:3
            //                    ^ref:1
            //                        ^ref:2
            b = a + b;
            //<ref:3
            //  ^ref:1
            //      ^ref:3
            a = b;
        }
    }
}
