// SPDX-License-Identifier: MIT
pragma solidity *;

bool constant K = true;

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}
