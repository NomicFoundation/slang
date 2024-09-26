// ---- path: counter.sol
struct Counter {
    uint value;
}

function increment(Counter memory _counter) public {}

using {increment} for Counter global;

// ---- path: main.sol
import {Counter} from "counter.sol";

contract Test {
    function test(Counter memory c) public {
        c.increment();
    }
}
