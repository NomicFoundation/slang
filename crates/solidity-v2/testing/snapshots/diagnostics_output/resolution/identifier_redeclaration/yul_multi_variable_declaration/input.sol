// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            function g() -> a, b {
                a := 1
                b := 2
            }
            let x, x := g()
        }
    }
}
