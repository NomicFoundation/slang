contract Test {
    function playground() public returns (int) {
        int x = 1;
        int y = x + 2;
        return (x + y * x) / y;
    }
}
