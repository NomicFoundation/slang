// --- path: main.sol
pragma solidity *;

import {Log as Renamed} from "./lib.sol";

contract Test {
    function test() public {
        emit Renamed({level: 1, name: "Testing"});
    }
}

// --- path: lib.sol
pragma solidity *;

event Log(string name, uint level);
