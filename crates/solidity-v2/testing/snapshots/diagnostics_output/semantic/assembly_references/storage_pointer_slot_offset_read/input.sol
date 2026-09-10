// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint[] x;

    function f() public view {
        uint[] storage y = x;
        assembly {
            let s := y.slot
            let o := y.offset
        }
    }
}
