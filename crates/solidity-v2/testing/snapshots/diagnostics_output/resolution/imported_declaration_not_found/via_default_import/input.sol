// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// 'Deep' is not declared in 'lib.sol', but 'lib.sol' pulls it in with a
// default import, so it is visible at its file scope.
import {Deep} from "./lib.sol";

contract Test is Deep {}

// --- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./deep.sol";

// --- path: deep.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

contract Deep {}
