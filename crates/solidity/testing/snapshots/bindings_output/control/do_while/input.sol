contract Test {
    function test() public {
        int i = 1;
        do {
            int odd = i % 2;
            i *= 3;
        } while (i < 100);
    }
}
