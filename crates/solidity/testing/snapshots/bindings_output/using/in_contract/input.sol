library Lib {
    struct Counter {
        uint value;
    }

    function increment(Counter memory _counter) public {}
}

contract Test {
    using Lib for Lib.Counter;

    function test(Lib.Counter memory c) public {
        c.increment();
    }
}
