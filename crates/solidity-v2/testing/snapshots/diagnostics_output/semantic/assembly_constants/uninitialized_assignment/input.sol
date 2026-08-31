// SPDX-License-Identifier: MIT
pragma solidity *;

// An uninitialized constant is rejected at its declaration, so the
// assignment gets no extra error.
contract C {
    uint256 public constant K;

    function f() public pure {
        assembly {
            K := 1
        }
    }
}
