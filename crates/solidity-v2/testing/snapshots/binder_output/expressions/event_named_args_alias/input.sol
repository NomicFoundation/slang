// --- path: main.sol
pragma solidity *;

import {Log as Renamed} from "./lib.sol";

contract Test {
    function test() public {
        // Invalid Solidity: an event invocation has to be prefixed by `emit`.
        // The argument names should still resolve to the event parameters.
        Renamed({level: 1, name: "Testing"});
    }
}

// --- path: lib.sol
pragma solidity *;

event Log(string name, uint level);
