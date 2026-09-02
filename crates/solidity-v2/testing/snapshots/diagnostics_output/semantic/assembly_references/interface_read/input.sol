// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {}

contract C {
    function f() public pure {
        assembly {
            let t := I
        }
    }
}
