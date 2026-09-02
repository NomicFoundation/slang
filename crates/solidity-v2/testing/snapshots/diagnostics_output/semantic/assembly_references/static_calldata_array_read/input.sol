// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint[3] calldata a) external pure {
        assembly {
            let t := a
        }
    }
}
