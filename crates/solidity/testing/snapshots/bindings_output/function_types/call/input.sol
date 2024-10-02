contract Test {
    struct Value {
        int x;
    }
    function test(function() returns (Value) f) public {
        f().x;
    }
}
