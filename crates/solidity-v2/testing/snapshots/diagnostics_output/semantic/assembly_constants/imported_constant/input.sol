// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./lib.sol";

// A computed value in another file is not a forward reference.
contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}

// ---- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant K = 1 + 1;
