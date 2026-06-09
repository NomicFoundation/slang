// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./constants.sol";

contract C {
    uint256[A] data;
}

// ---- path: constants.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant A = B;
uint256 constant B = A;
