// --- path: a.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant k = 1;

// --- path: b.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// Recovered from solc:
// test/libsolidity/syntaxTests/functionCalls/module_not_callable.sol
// Calling an import alias, which names a source unit and not a callable.

import "a.sol" as A;

contract C {
    // TypeError 5704: This expression is not callable.
    uint256 a = A(1000);

    // Selecting a member of the same alias is fine.
    uint256 b = A.k;
}
