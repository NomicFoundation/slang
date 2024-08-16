contract Test {
    function test() public {
        int b = 1;
        for ((int i, int a) = (0, b); i < 10; i++) {
            b = a + b;
            a = b;
        }
    }
}
