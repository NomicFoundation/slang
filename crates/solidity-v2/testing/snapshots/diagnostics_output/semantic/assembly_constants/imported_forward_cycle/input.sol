// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./lib.sol";

// A cyclic import makes a cross-file forward reference possible. The forward
// test is same file only, so the read is accepted here.
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

import "./main.sol";

uint256 constant K = 1 + 1;
