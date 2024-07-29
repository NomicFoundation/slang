contract Test {
    function playground() returns (int) {
        int x = 1;
        //  ^def:1
        int y = 2;
        //  ^def:2
        int z = Utils.adder(x, y);
        //                     ^ref:2
        //                  ^ref:1
        //            ^ref:6
        //      ^ref:5
        //  ^def:3
        pointless(z, x);
        //<ref:4
        //           ^ref:1
        //        ^ref:3
    }

    function pointless(int x, int y) {}
    //       ^def:4
}

library Utils {
    //  ^def:5
    function adder(int a, int b) returns (int) {
        //   ^def:6
        return a + b;
    }
}
