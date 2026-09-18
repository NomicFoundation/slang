// SPDX-License-Identifier: MIT
pragma solidity *;

// Companion to `yul_variable_shadows_blobhash`: every version runs at Cancun,
// the target that makes `blobhash` a Solidity global and a Yul built-in. Before
// 0.8.24, the version that introduces it, the declaration must be accepted,
// because the target alone may not promote a name the language version doesn't
// know. From 0.8.24 both gates are satisfied and it is rejected. solc declines
// the Cancun target before 0.8.24, which is the declared divergence.
contract C {
    function f() public pure {
        assembly {
            let blobhash := 1
        }
    }
}
