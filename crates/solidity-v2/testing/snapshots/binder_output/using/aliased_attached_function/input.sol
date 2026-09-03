// ---- path: main.sol
pragma solidity *;

import {g as f} from "other.sol";

using {f} for uint;

contract Test {
    function test(uint a) public {
        a.f();
    }
}

// ---- path: other.sol
pragma solidity *;

function g(uint a) {}
