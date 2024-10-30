contract Base {
    struct Value { int x; }
    Value public value;
}
contract Test {
    function test(Base base) public {
        base.value().x;
    }
}
