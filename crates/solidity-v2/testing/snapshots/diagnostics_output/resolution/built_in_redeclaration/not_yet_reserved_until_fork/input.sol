// SPDX-License-Identifier: MIT
pragma solidity *;

// `mcopy` becomes a Yul built-in in 0.8.24, but only on Cancun and later. Each
// version below is analyzed at the EVM target its own `solc` defaults to, and
// 0.8.24 still defaults to Shanghai — so the declaration is accepted up to and
// including 0.8.24, and rejected from 0.8.25 on, where the default becomes
// Cancun. While the built-in is unavailable solc merely warns that the name
// "will be promoted to Yul reserved identifier in the future", which does not
// reject the input.
contract C {
    function f() public pure {
        assembly {
            let mcopy := 1
        }
    }
}
