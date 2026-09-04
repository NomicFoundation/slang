// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant K = type(uint256).max;

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}
