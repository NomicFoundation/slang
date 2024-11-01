library Lib {
    struct Value {
        int x;
    }
    function getValue() external returns (Value memory) {}
    function use(Value memory x) external {}
}
contract Test {
    using Lib for Lib.Value;
    function test() internal {
        Lib.getValue().use();
    }
}
