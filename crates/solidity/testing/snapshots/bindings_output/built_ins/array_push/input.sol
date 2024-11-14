contract Test {
    struct Data { uint value; }
    Data[] values;
    function test() public {
        values.push().value = 1;
    }
}
