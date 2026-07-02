// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            function g() -> add {
                add := 1
            }
        }
    }
}
