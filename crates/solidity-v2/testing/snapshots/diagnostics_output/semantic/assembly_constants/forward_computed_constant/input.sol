// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}

uint256 constant K = 1 + 1;
