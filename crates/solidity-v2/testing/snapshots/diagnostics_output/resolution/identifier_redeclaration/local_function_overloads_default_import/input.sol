// --- path: s1.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function f(uint24) pure returns (uint) {
    return 24;
}

// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "s1.sol";

// A locally-declared free function may overload a function brought into scope
// by a default import, so this is not a redeclaration.
function f(bool) pure returns (bool) {
    return true;
}
