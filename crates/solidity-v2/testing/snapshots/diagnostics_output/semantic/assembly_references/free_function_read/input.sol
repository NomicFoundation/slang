// SPDX-License-Identifier: MIT
pragma solidity *;

function ff() pure {}

contract C {
    function f() public pure {
        assembly {
            let t := ff
        }
    }
}
