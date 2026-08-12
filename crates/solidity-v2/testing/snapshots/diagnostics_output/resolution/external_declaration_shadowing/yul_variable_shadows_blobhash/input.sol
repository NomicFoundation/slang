// SPDX-License-Identifier: MIT
pragma solidity *;

// `blobhash` becomes a Solidity global and a Yul built-in only from
// 0.8.24/Cancun. Before Cancun it is neither, so `let blobhash := 1` is a valid
// Yul identifier and must be accepted; from Cancun on it is rejected as a
// built-in redeclaration.
contract C {
    function f() public pure {
        assembly {
            let blobhash := 1
        }
    }
}
