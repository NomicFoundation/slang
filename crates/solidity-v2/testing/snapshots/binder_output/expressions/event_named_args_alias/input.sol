// --- path: main.sol
import {Log as Renamed} from "./lib.sol";

contract Test {
    function test() public {
        // Invalid Solidity: an event invocation has to be prefixed by `emit`.
        // The argument names should still resolve to the event parameters.
        Renamed({level: 1, name: "Testing"});
    }
}

// --- path: lib.sol
event Log(string name, uint level);
