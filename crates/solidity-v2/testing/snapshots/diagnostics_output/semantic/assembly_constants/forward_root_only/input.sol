// SPDX-License-Identifier: MIT
pragma solidity *;

// Only the root of the alias chain is position tested. `A` is declared
// before the assembly block, but its root `B` is not.
uint256 constant A = B;

contract C {
    function f() public pure {
        assembly {
            let x := A
        }
    }
}

uint256 constant B = 1 + 1;
