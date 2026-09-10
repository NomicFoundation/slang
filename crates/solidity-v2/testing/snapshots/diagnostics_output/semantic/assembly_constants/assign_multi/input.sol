// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant A = 1;
uint256 constant B = 2;

contract C {
    function f() public pure {
        assembly {
            function g() -> x, y {
                x := 1
                y := 2
            }
            A, B := g()
        }
    }
}
