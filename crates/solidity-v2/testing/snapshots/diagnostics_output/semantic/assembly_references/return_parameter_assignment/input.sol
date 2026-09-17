// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure returns (uint r) {
        assembly {
            r := 1
        }
    }
}
