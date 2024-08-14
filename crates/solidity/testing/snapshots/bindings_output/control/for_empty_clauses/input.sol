contract Test {
    function test() public {
        int i = 1;
        for (;;) {
            if (i > 10) break;
            i++;
        }
    }
}
