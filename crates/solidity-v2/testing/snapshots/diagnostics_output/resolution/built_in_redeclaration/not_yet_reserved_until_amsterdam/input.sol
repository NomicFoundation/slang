// SPDX-License-Identifier: MIT
pragma solidity *;

// The companion `reserved_on_amsterdam_target` covers the
// rejecting side.
contract C {
    function f() public pure {
        assembly {
            let slotnum := 1
        }
    }
}
