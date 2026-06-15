// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            // Variables declared in the for initialization block are
            // visible (and cannot be redeclared) in the condition,
            // iterator and body blocks.
            for { let x := 0 } lt(x, 2) { x := add(x, 1) } {
                let x := 1
            }
        }
    }
}
