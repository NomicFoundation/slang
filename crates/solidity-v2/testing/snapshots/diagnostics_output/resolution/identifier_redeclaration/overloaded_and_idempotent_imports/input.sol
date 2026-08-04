// --- path: s1.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function f(uint24) pure returns (uint) {
    return 24;
}

// --- path: s2.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function f(bool) pure returns (bool) {
    return true;
}

// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// `f` is imported three times: the first two name the very same declaration
// (idempotent re-import), and the third names a different declaration that
// legally overloads it. None of these is a redeclaration.
import {f} from "s1.sol";
import {f as f} from "s1.sol";
import {f} from "s2.sol";
