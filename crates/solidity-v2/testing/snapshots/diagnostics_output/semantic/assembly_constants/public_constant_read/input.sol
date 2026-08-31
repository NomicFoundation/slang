// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256 public constant K = 41;

    function f() public pure {
        assembly {
            let x := K
        }
    }
}
