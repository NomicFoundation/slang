// --- path: input.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./other.sol" as other;

contract C {
    function f() public {
        emit other.Nope();
    }
}

// --- path: other.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function g() pure {}
