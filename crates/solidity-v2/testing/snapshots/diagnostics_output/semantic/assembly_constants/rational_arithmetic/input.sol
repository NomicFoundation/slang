// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant K = 1 + 2 * 3;

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}
