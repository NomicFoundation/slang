// SPDX-License-Identifier: MIT
pragma solidity *;

bytes constant B = hex"1122";

contract C {
    function f() public pure {
        assembly {
            let x := B
        }
    }
}
