contract Test {
    int y;
    function someFunc(int x) public returns (int) {
        return add(x, y);
    }
    function add(int, int) public returns (int) {}
}
