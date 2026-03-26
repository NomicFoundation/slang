contract Test {
    function test_for_empty_init() public {
        int i = 1;
        int x = 0;
        for (; i < 10; i++) {
            x += i;
        }
    }
}
