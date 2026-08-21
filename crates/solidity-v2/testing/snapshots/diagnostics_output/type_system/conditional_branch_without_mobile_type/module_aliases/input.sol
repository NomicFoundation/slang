// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./lib.sol" as A;
import "./lib.sol" as B;

// `A` and `B` are distinct aliases of one source unit, and both branches name a
// module rather than a value. solc accepts this.

function pick() pure returns (uint256) {
    return (true ? A : B).K;
}

// ---- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant K = 1;
