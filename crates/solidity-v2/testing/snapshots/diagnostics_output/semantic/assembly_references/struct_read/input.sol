// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct S {
        uint a;
    }

    function f() public pure {
        assembly {
            let t := S
        }
    }
}
