interface TestBase {
    function add(int, int) external returns (int);

    function add(int) external returns (int);
}

contract Test {
    int y;

    function someFunc(TestBase[] memory arr, int x) internal {
        arr[0].add(x);
        arr[0].add(x, y);
    }
}
