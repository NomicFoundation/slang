// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            function g() {
                // The function's own name is visible (and reserved)
                // inside its body.
                let g := 0
            }
        }
    }
}
