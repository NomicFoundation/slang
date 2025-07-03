contract Test {
    struct Value {
        int x;
    }
    function test(function() returns (Value memory) f) internal {
        f().x;
    }
}
