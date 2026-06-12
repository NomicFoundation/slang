// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            {
                let g := 0
            }
            // Hoisted function names clash with variables even in
            // sibling subscopes.
            function g() {}
        }
    }
}
