// --- path: main.sol
pragma solidity *;

import {Failure as Renamed} from "./lib.sol";

contract Test {
    function test() public {
        revert Renamed({level: 1, name: "Testing"});
    }
}

// --- path: lib.sol
pragma solidity *;

error Failure(string name, uint level);
