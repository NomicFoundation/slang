// SPDX-License-Identifier: MIT
pragma solidity *;

address constant K = 0x1111111111111111111111111111111111111111;

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}
