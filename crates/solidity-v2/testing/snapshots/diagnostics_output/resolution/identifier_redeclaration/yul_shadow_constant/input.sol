// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint constant a = 42;

    function f() public pure {
        assembly {
            let a := 1
        }
    }
}
