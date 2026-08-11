// SPDX-License-Identifier: MIT
pragma solidity *;

// Companion to `not_yet_reserved_until_fork`, one version lower. `mcopy` is
// introduced in 0.8.24, so at 0.8.23 it is not a built-in on any EVM target and
// the declaration below must be accepted everywhere — including Cancun and
// later, where the newer version would reject it.
contract C {
    function f() public pure {
        assembly {
            let mcopy := 1
        }
    }
}
