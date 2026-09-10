// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant K = 1 ether;

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}
