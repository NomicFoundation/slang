// SPDX-License-Identifier: MIT
pragma solidity *;

// A built-in that is not available yet does not reserve its name, so declaring
// it is legal and a later reference must resolve to that declaration. `mcopy`
// needs both 0.8.24 and a Cancun target, and 0.8.24 still defaults to Shanghai,
// so both lines below are accepted up to 0.8.24; from 0.8.25, where the default
// target is Cancun, the declaration is rejected as a built-in redeclaration.
contract C {
    function f() public pure {
        assembly {
            let mcopy := 1
            let x := mcopy
        }
    }
}
