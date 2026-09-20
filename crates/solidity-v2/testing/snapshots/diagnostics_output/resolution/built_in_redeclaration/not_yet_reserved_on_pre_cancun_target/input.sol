// SPDX-License-Identifier: MIT
pragma solidity *;

// The sibling `not_yet_reserved_until_fork` runs at each version's default
// target, where the last version that still accepts the name is 0.8.24. Pinning
// a pre-Cancun target every supported `solc` accepts asserts the same thing at
// every language version instead: while `mcopy` is not available, its name is
// free to declare, even on a compiler that has known the built-in for versions.
// From 0.8.35 solc warns that the name will be promoted to a Yul reserved
// identifier, which slang has no equivalent for, but neither side rejects it.
contract C {
    function f() public pure {
        assembly {
            let mcopy := 1
        }
    }
}
