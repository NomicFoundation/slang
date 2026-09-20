// SPDX-License-Identifier: MIT
pragma solidity *;

// `blobhash` becomes a Solidity global and a Yul built-in in 0.8.24, but only
// on Cancun and later. Since 0.8.24 still defaults to Shanghai, `blobhash` is
// an ordinary Yul identifier up to and including 0.8.24 and the declaration
// must be accepted; from 0.8.25, where the default target is Cancun, it is
// rejected as a built-in redeclaration.
contract C {
    function f() public pure {
        assembly {
            let blobhash := 1
        }
    }
}
