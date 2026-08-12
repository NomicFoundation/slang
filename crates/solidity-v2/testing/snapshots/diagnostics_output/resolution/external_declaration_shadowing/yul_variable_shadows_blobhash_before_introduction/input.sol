// SPDX-License-Identifier: MIT
pragma solidity *;

// Companion to `yul_variable_shadows_blobhash`, pinned one version lower.
// `blobhash` is introduced in 0.8.24, so at 0.8.23 it is neither a Solidity
// global nor a Yul built-in on *any* EVM target: `let blobhash := 1` must be
// accepted everywhere.
contract C {
    function f() public pure {
        assembly {
            let blobhash := 1
        }
    }
}
