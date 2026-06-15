// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            function g() -> v {
                v := 1
            }
            // Yul functions cannot be overloaded.
            function g(a) -> v {
                v := a
            }
        }
    }
}
