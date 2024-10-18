contract CustomArrays {
    struct Value {
        uint value;
    }

    Value[] values;
    Value[][5] matrix;

    function test() public {
        matrix[0][1].value += values[1].value;
    }
}
