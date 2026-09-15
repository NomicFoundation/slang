// SPDX-License-Identifier: MIT
pragma solidity *;

// The assignment is rejected before the alias chain is inspected, so the
// cycle is additionally reported at the declarations.
uint256 constant A = B;
uint256 constant B = A;

contract C {
    function f() public pure {
        assembly {
            A := 1
        }
    }
}
