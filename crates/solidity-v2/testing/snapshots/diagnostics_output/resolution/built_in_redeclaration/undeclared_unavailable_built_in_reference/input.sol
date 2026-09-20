// SPDX-License-Identifier: MIT
pragma solidity *;

// A reference with no declaration in scope should still fail with a "built-in
// not available" diagnostic if the built-in exists in later versions or on
// later targets. `mcopy` needs both 0.8.24 and a Cancun target, so before
// 0.8.24 both gates are reported, at 0.8.24 (whose default target is Shanghai)
// only the target one, and from 0.8.25 the call resolves.
contract C {
    function f() public pure {
        assembly {
            mcopy(1, 2, 3)
        }
    }
}
