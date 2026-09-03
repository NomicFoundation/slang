// ---- path: main.sol
pragma solidity *;

import {Base} from "./lib.sol";

contract Test is Base {
    function test() public Base.beforeRun {
    }
}


// ---- path: lib.sol
pragma solidity *;

contract Base {
    modifier beforeRun() {
        _;
    }
}
