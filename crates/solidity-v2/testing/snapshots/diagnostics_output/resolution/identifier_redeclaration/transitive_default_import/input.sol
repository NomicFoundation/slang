// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./middle.sol";

contract X {}

// --- path: middle.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./leaf.sol";

// --- path: leaf.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

contract X {}
