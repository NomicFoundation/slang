contract Test {
    struct Value {
        uint value;
    }
    mapping(uint => Value) values;

    function test(uint _id) public returns (uint) {
        return values[_id].value;
    }
}
