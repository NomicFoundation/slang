interface External {
    struct Data { uint value; }
    function sample() external payable returns (Data memory);
}

contract Test {
    function test(External ext) public returns (uint) {
        return ext.sample{ value: 10, gas: 20 }().value;
    }
}
