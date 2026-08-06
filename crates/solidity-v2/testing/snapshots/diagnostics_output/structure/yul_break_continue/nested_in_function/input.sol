// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            // A for-loop does not extend into a function declared in its body,
            // so `break` here is treated as outside any for-loop body.
            for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                function g() { break }
            }
        }
    }
}
