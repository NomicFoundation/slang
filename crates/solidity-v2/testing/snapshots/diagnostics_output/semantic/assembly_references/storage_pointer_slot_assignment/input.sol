// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint[] x;

    function f() public view {
        uint[] storage y = x;
        assembly {
            y.slot := 1
        }
    }
}
