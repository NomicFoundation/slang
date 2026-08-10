// --- path: base.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function duplicated(uint256 a) pure returns (uint256) {
    return a;
}

function ok(uint256 a) pure returns (uint256) {
    return a;
}

// --- path: derived.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./base.sol";

// Redeclares a free function the unqualified import already brought into scope.
function duplicated(uint256 b) pure returns (uint256) {
    return b + 1;
}

// Distinguishable from the imported `ok`, so it just joins the overload set.
function ok(string memory a) pure returns (uint256) {
    return bytes(a).length;
}
