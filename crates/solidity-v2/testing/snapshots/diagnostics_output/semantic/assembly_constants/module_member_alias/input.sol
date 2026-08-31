// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./lib.sol" as M;

// Member access is not an alias edge, so the chain roots at `K` and its
// value is a typed expression.
uint256 constant K = M.C2;

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}

// ---- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant C2 = 42;
