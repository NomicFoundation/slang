// --- path: main.sol
import {Failure as Renamed} from "./lib.sol";

contract Test {
    function test() public {
        revert Renamed({level: 1, name: "Testing"});
    }
}

// --- path: lib.sol
error Failure(string name, uint level);
