library Test {
    enum Choice { Yes, No }

    function test() public {
        (int x, int y) = (1, 2);
        (int z, , int w) = (1, 2, 3);
        assert(x == z);
        assert(y < w);
        (, Choice c) = (Choice.Yes, Choice.No);

        assert(c == Choice.No);
    }
}
