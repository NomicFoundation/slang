// --- path: input.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./other.sol" as other;

contract C {
    function missing() internal pure {
        other.nope();
    }

    function present() internal pure {
        other.f();
    }
}

// --- path: other.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function f() pure {}
