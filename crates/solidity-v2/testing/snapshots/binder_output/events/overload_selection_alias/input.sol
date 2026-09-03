// --- path: main.sol
pragma solidity *;

// Both events are imported under the same name, so `Renamed` is an ambiguous
// alias that needs to be disambiguated by the arguments of the `emit`.
import {Pair as Renamed, Flag as Renamed} from "./lib.sol";

contract Test {
    function test() internal {
        emit Renamed(1, 2);
        emit Renamed(false);

        emit Renamed({x: 1, y: 2});
        emit Renamed({flag: true});
    }
}

// --- path: lib.sol
pragma solidity *;

event Pair(uint x, uint y);
event Flag(bool flag);
