// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() internal pure {}

    function f() public pure {
        assembly {
            let t := g
        }
    }
}
