// --- path: s1.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function f(uint24) pure returns (uint) {
    return 24;
}

// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// `f` is named twice by the same import deconstruction. Both occurrences refer
// to the very same declaration, so this is idempotent and not a redeclaration.
import {f, f} from "s1.sol";
