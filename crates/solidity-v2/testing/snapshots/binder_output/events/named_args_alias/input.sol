// --- path: main.sol
import {Log as Renamed} from "./lib.sol";

contract Test {
    function test() public {
        emit Renamed({level: 1, name: "Testing"});
    }
}

// --- path: lib.sol
event Log(string name, uint level);
