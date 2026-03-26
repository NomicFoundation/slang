contract Test {
    function test() public {
        int x = 1;
        for (int i = 0;; i++) {
            x = x * 2;
            if (i > 10) break;
        }
    }
}
