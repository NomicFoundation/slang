// SPDX-License-Identifier: MIT
pragma solidity *;

string constant S = "abc";

contract C {
    function f() public pure {
        assembly {
            let x := S
        }
    }
}
