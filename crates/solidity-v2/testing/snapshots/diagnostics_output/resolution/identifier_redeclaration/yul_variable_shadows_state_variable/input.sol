// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256 x;

    // A Yul `let` variable declared inside a Yul function may not shadow a
    // Solidity declaration visible at the assembly site, here the state
    // variable `x`. The search walks out of the function scope to find it.
    function f() public pure {
        assembly {
            function g() {
                let x := 1
                pop(x)
            }
        }
    }
}
