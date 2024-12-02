contract Test {
    struct Value {
        int x;
    }
    function test() public {
        Value(10).x;
        Value({x: 10}).x;
    }
}
