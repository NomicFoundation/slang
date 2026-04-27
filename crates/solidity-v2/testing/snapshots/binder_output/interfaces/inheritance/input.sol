interface Counter {
    enum Type { up, down }
    function count() external returns (uint);
}

interface MutableCounter is Counter {
    function increment() external;
}

contract Test {
    function test(address _counter) public returns (uint) {
        MutableCounter m = MutableCounter(_counter);
        MutableCounter.Type t;
        m.increment();
        return m.count();
    }
}
