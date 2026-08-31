// SPDX-License-Identifier: MIT
pragma solidity *;

bytes32 constant K = "abc";

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}
