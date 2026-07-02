// --- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./a.sol";

contract C {
    // A Yul `let` variable may not shadow a symbol brought into scope by a
    // default import (`import "./a.sol";`), even though `foo` is declared in
    // another file and has no local definition here.
    function f() public pure {
        assembly {
            let foo := 1
            pop(foo)
        }
    }
}

// --- path: a.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function foo() pure returns (uint256) {
    return 1;
}
