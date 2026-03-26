contract Test {
    int y;
    function someFunc(int x) returns (int) {
        return add(x, y);
    }
    function add(int, int) returns (int);
}
