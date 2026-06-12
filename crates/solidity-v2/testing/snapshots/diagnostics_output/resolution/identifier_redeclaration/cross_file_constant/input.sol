// --- path: a.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./b.sol";

uint constant c = 7;

// --- path: b.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./a.sol";

uint constant c = 8;
