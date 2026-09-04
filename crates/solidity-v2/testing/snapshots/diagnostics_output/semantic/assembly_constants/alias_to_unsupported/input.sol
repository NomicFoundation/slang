// SPDX-License-Identifier: MIT
pragma solidity *;

bytes32 constant H = keccak256("x");
bytes32 constant A = H;

contract C {
    function f() public pure {
        assembly {
            let x := A
        }
    }
}
