contract Test {
    function someFunc(int x) public returns (int) {
        return divide({ dividend: x, divisor: 3 });
    }
    function divide(int dividend, int divisor) public returns (int);
}
