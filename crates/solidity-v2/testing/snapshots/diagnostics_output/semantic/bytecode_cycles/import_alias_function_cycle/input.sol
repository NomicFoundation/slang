// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./other.sol" as M;

contract A {
    function f() public {
        M.create();
    }
}

// ---- path: other.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./main.sol";

function create() {
    new A();
}
