// --- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function f(uint256) pure returns (uint256) {
    return 1;
}

// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {f} from "lib.sol";

// A locally-declared free function may overload an imported function, so this
// is not a redeclaration.
function f(bool) pure returns (bool) {
    return true;
}
