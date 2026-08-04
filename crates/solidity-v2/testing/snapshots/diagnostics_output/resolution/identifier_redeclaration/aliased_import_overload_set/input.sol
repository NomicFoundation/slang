// --- path: s1.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function f(uint24) pure returns (uint) {
    return 24;
}

function g(bool) pure returns (bool) {
    return true;
}

// --- path: s2.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// `f` and `g` are both aliased to the name `g`. Since both refer to free
// functions, they form an overload set rather than a redeclaration, and must
// not be reported as an error. See solc's
// `semanticTests/multiSource/free_different_interger_types.sol`.
import {f as g, g as g} from "s1.sol";
