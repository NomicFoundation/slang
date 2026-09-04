// SPDX-License-Identifier: MIT
pragma solidity *;

// The assignment error wins over the suffix error.
uint256 constant K = 41;

contract C {
    function f() public pure {
        assembly {
            K.slot := 1
        }
    }
}
