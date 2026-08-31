// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant K = 41;

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}
