interface Counter {
    function count() external returns (uint);
}

contract Test {
    function test(address _counter) public returns (uint) {
        return Counter(_counter).count();
    }
}
