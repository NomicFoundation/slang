// SPDX-License-Identifier: MIT
pragma solidity *;

// `difficulty` is a Yul built-in only up to Paris (`evm_enabled = Till(Paris)`),
// yet its name stays reserved on every target: pre-Paris it is rejected as a
// built-in, and from Paris on — where the built-in no longer exists — it is
// still rejected as a reserved identifier. So the declaration below is rejected
// on every target, via two different mechanisms across the Paris boundary.
contract C {
    function f() public pure {
        assembly {
            let difficulty := 1
        }
    }
}
