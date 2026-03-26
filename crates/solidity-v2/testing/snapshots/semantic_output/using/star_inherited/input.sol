library Lib {
    function increment(uint x) public {}
}

contract Base {
    using Lib for *;
}

contract Test is Base {
    function test(uint x) public {
        // should resolve for Solidity < 0.7.0
        x.increment();
    }
}
