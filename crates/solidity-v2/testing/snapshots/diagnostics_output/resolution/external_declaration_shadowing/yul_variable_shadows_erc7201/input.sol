// SPDX-License-Identifier: MIT
pragma solidity *;

// `erc7201` becomes a Solidity global only in 0.8.35, so before that the name
// can be used as a valid Yul identifier.
contract C {
    function f() public pure {
        assembly {
            let erc7201 := 1
        }
    }
}
