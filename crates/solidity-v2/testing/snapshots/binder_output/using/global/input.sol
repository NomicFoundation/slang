// ---- path: counter.sol
pragma solidity *;

struct Counter {
    uint value;
}

function increment(Counter memory _counter) {}

using {increment} for Counter global;

// ---- path: main.sol
pragma solidity *;

import {Counter} from "counter.sol";

contract Test {
    function test(Counter memory c) public {
        c.increment();
    }
}
