// SPDX-License-Identifier: MIT
pragma solidity *;

// The alias chain steps to an uninitialized constant, which is not a valid
// chain link, so the chain has no root.
contract C {
    uint256 public constant U;
    uint256 constant A = U;

    function f() public pure {
        assembly {
            let x := A
        }
    }
}
