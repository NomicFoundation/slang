// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint x;

    function f() public pure {
        assembly {
            let s := x.slot
            let o := x.offset
        }
    }
}
