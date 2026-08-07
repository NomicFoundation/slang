// SPDX-License-Identifier: MIT
pragma solidity *;

// `mcopy` is one of the grace-period built-in names solc has not promoted to a
// reserved identifier yet, so the declaration below is only rejected once
// Cancun (which introduces the built-in) is targeted. Before that solc merely
// warns that the name "will be promoted to Yul reserved identifier in the
// future", which does not reject the input, so it must be accepted.
contract C {
    function f() public pure {
        assembly {
            let mcopy := 1
        }
    }
}
