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

// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// `f` is brought into scope by both default imports — through "s2.sol"
// transitively and through "s1.sol" directly. Both refer to the very same
// declaration, so this is not a redeclaration.
import "s2.sol";
import "s1.sol";
