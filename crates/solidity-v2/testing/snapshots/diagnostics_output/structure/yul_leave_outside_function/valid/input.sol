// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            function g() {
                // `leave` is allowed anywhere inside a function body, including
                // nested blocks and loops.
                let x := 0
                if gt(x, 5) { leave }
                for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                    if gt(i, 5) { leave }
                }
            }
            g()
        }
    }
}
