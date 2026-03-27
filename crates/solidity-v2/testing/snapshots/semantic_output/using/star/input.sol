library Lib {
    function increment(uint x) public {}
}

contract Test {
    using Lib for *;

    function test(uint x) public {
        x.increment();
    }
}
