// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            let g := 2
            function g() {}
        }
    }
}
