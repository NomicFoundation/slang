// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./constants.sol" as M;

uint256 constant X = M.A;

// ---- path: constants.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {X} from "./main.sol";

uint256 constant A = X;
