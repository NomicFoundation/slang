// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public view {
        C c = this;
        assembly {
            let t := c
        }
    }
}
