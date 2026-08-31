// SPDX-License-Identifier: MIT
pragma solidity *;

int256 constant K = -1;

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}
