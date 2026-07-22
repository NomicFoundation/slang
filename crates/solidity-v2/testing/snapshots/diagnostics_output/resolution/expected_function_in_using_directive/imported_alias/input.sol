// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {original as aliased} from "./lib.sol";

contract C {
    using {aliased} for uint256;
}

// --- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function original(uint256 self) pure returns (uint256) {
    return self;
}
