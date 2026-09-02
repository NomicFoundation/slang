// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./lib.sol" as M;

contract C {
    function f() public pure {
        assembly {
            let t := M
        }
    }
}

// ---- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant K = 1;
