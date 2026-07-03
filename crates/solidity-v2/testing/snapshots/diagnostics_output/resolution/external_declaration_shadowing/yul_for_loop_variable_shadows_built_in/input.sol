// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            for { let block := 0 } lt(block, 10) { block := add(block, 1) } {}
        }
    }
}
