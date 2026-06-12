// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            let x := 1
            function g() -> x {
                x := 2
            }
        }
    }
}
