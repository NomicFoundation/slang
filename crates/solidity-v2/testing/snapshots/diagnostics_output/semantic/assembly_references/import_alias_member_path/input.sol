// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./lib.sol" as M;

// The second item is not a suffix name, and the alias takes no suffix anyway.
contract C {
    function f() public pure {
        assembly {
            let t := M.K
        }
    }
}

// ---- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant K = 1;
