// --- path: declarations.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function f() pure returns (uint256) {
    return 1337;
}

function g() pure returns (uint256) {
    return 42;
}

// --- path: alias.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {f as g} from "./declarations.sol";

// --- path: consumer.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// Brings in `f` renamed to `g`...
import "./alias.sol";
// ...alongside the original `g`, which it cannot be told apart from.
import "./declarations.sol";

contract C {
    function callIt() public pure returns (uint256) {
        return g();
    }
}
