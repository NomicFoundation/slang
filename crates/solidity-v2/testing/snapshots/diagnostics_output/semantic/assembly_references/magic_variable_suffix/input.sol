// SPDX-License-Identifier: MIT
pragma solidity *;

// Solidity globals are not looked up from inline assembly, so a suffix on one
// is not reported yet.
contract C {
    function f() public pure {
        assembly {
            let a := msg.slot
            let b := block.offset
            let c := tx.length
        }
    }
}
