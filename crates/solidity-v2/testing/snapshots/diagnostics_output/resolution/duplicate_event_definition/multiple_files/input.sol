// ---- path: a.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

event E();

// ---- path: b.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "a.sol";

event E();
