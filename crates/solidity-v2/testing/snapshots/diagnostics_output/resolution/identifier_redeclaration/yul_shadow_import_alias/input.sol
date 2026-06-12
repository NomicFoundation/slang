// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {A as b} from "./a.sol";

contract B {
    function f() public pure {
        assembly {
            let b := 3
        }
    }
}

// --- path: a.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {}
