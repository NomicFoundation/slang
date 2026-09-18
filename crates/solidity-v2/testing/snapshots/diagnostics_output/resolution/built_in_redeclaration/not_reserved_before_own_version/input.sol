// SPDX-License-Identifier: MIT
pragma solidity *;

// Companion to `not_yet_reserved_until_fork`: every version runs at Cancun, the
// target that makes `mcopy` a built-in. Before 0.8.24, the version that
// introduces it, the declaration must be accepted, because a built-in the
// language version doesn't have yet reserves nothing — the target alone may not
// promote the name. From 0.8.24 both gates are satisfied and it is rejected.
// solc declines the Cancun target before 0.8.24, which is the declared
// divergence.
contract C {
    function f() public pure {
        assembly {
            let mcopy := 1
        }
    }
}
