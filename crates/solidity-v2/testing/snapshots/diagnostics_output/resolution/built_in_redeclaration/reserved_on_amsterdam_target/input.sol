// SPDX-License-Identifier: MIT
pragma solidity *;

// Companion to `not_yet_reserved_until_amsterdam`, which runs at each version's
// default target and so only ever shows the name free. Targeting Amsterdam,
// where `slotnum` becomes a Yul built-in, asserts that the same declaration is
// then rejected.
contract C {
    function f() public pure {
        assembly {
            let slotnum := 1
        }
    }
}
