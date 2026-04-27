contract Test {
    function test() public returns (int) {
        int x = 1;
        int y = 2;
        if (x > 1) {
            int z = 3;
            y = x + 10;
        } else {
            int w = 4;
            y = x + 20;
        }
        int r = x + y;
        return r;
    }
}
