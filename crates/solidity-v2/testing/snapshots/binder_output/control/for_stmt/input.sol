contract Test {
    function test() public {
        int x = 1;
        int y = 0;
        for (int i = x - 1; i < 10 && x < 500; i++) {
            x = x * 2;
            y = y + i;
        }
    }
}
