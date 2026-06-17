// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./constants.sol";

contract Example {
    // The length references the imported constant `LEN`, whose value
    // expression `~B` (in constants.sol) is where the overflow actually
    // happens. The error should point at `~B` in constants.sol, not at this
    // use site.
    uint256[LEN] x;
}

// --- path: constants.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant B = 3;

// `~B` folds to a negative value that does not fit the unsigned result type.
uint256 constant LEN = ~B;
