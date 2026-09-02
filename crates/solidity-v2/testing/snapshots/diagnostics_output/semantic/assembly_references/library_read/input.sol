// SPDX-License-Identifier: MIT
pragma solidity *;

library L {}

contract C {
    function f() public pure {
        assembly {
            let t := L
        }
    }
}
