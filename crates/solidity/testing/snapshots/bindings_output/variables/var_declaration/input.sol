contract Test {
    struct Value { int x; }
    function test(Value memory value) public {
        var v = value;
        v.x;
    }
}
