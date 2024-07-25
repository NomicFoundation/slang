library Test {
    enum Choice { Yes, No }
    //   ^def:1

    function test() public pure {
        (int x, int y) = (1, 2);
        //          ^def:5
        //   ^def:4
        (int z, , int w) = (1, 2, 3);
        //            ^def:7
        //   ^def:6
        assert(x == z);
        //          ^ref:6
        //     ^ref:4
        assert(y < w);
        //         ^ref:7
        //     ^ref:5
        (, Choice c) = (Choice.Yes, Choice.No);
        //        ^def:8
        // ^ref:1

        assert(c == Choice.No);
        //     ^ref:8
    }
}
