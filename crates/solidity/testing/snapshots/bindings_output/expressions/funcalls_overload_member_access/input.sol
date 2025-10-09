interface Overloaded {
    function add(int64, int64) external returns (Overloaded);

    function add(int64) external returns (Overloaded);
}

contract Test {
    function someFunc(Overloaded x, int64 y) internal {
        x.add(y).add(y, y);
    }
}
