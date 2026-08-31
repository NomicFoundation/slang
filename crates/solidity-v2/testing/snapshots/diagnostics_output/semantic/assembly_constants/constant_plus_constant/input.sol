// SPDX-License-Identifier: MIT
pragma solidity *;

// A constant operand pins the value to its declared type, so the result is
// not an untyped literal even though it is a compile time constant.
uint256 constant A = 1;
uint256 constant B = A + 1;

contract C {
    function f() public pure {
        assembly {
            let x := B
        }
    }
}
