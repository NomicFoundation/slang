// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./other.sol" as M;

contract C {
    M.A.S s;
}

// --- path: other.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {}
