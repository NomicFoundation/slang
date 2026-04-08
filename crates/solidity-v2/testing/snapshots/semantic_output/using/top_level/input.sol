struct Counter {
    uint value;
}

function increment(Counter memory _counter) public {}

using {increment} for Counter;

contract Test {
    function test(Counter memory c) public {
        c.increment();
    }
}
