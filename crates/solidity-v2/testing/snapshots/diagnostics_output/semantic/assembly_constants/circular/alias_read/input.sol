// SPDX-License-Identifier: MIT
pragma solidity *;

// The cycle is reported at each declaration, so the assembly reference
// itself needs no extra error.
uint256 constant A = B;
uint256 constant B = A;

contract C {
    function f() public pure {
        assembly {
            let x := A
        }
    }
}
