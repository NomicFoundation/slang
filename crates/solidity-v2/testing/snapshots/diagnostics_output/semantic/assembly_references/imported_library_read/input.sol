// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {L} from "./lib.sol";

contract C {
    function f() public pure {
        assembly {
            let t := L
        }
    }
}

// ---- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

library L {}
