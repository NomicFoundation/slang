// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant K = uint256(1) + 1;

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}
