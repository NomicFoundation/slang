contract Test {
    function someFunc(int x) returns (int) {
        return divide({ dividend: x, divisor: 3 });
    }
    function divide(int dividend, int divisor) returns (int);
}
