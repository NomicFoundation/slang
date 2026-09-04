// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant A = 41;
uint256 constant B = A;

contract C {
    function f() public pure {
        assembly {
            let x := B
        }
    }
}
