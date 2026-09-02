// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        uint a = 1;
        assembly {
            let t := a.slot
        }
    }
}
