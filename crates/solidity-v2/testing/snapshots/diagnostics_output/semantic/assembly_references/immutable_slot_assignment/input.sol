// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint immutable x;

    constructor() {
        x = 1;
    }

    function f() public {
        assembly {
            x.slot := 2
        }
    }
}
