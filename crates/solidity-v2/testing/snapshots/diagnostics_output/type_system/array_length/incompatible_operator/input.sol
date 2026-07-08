// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./constants.sol";

contract Example {
    // The length references the imported constant `LEN`, whose value
    // expression `(0 - 7) / B` (in constants.sol) is where the incompatible
    // operator actually occurs. The error should point there, not at this use
    // site.
    uint256[LEN] x;
}

// --- path: constants.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant B = 2;

// A negative literal has no common type with the unsigned constant `B`, so the
// division operator has no result type.
uint256 constant LEN = (0 - 7) / B;
