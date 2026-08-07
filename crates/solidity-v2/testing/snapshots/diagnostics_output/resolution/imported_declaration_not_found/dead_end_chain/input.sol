// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// 'Missing' is re-exported down a chain of imports that never reaches a
// declaration, so every hop in the chain is in error.
import {Missing} from "./lib.sol";

// --- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {Missing} from "./deep.sol";

// --- path: deep.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

contract Present {}
