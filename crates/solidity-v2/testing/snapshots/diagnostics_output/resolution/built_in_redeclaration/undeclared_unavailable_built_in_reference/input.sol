// SPDX-License-Identifier: MIT
pragma solidity *;

// A reference with no declaration in scope should still fail to 
// builtin not available if it's available in future versions.
contract C {
    function f() public pure {
        assembly {
            mcopy(1, 2, 3)
        }
    }
}
