// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./constants.sol";

// The base slot references the imported constant `SLOT`, whose value
// expression `~B` (in constants.sol) is where the overflow actually happens.
// The error should point at `~B` in constants.sol, not at this use site.
contract Example layout at SLOT {}

// --- path: constants.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant B = 3;

// `~B` folds to a negative value that does not fit the unsigned result type.
uint256 constant SLOT = ~B;
