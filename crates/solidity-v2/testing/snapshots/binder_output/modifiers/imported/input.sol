// ---- path: main.sol
import {Base} from "./lib.sol";

contract Test is Base {
    function test() public Base.beforeRun {
    }
}


// ---- path: lib.sol
contract Base {
    modifier beforeRun() {
        _;
    }
}
