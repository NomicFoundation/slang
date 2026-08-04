// --- path: s1.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function f() pure returns (uint) {
    return 1337;
}

// --- path: s2.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "s1.sol";

// --- path: s3.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// `f` is visible here both transitively through the default import of "s2.sol"
// (which itself imports "s1.sol") and through the explicit named import below.
// Both refer to the very same declaration, so re-importing it is idempotent and
// must not be reported as a redeclaration. See solc's
// `semanticTests/multiSource/free_function_transitive_import.sol`.
import "s2.sol";
import {f as f} from "s2.sol";
