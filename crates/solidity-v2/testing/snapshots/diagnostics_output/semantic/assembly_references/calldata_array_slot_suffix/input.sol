// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint[] calldata a) external pure {
        assembly {
            let t := a.slot
        }
    }
}
