// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256 x = 5;
    uint256 constant K = x;

    function f() public pure {
        assembly {
            let y := K
        }
    }
}
