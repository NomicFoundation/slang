// SPDX-License-Identifier: MIT
pragma solidity *;

// Companion to `blobhash`, which runs at each version's default target and so
// isolates the target gate at 0.8.24 alone, where the default is still
// Shanghai. Constraining every version to a pre-Cancun target asserts the same
// gate at every version instead of at one the defaults table happens to leave
// behind.
contract Foo {
    function f() public view returns (bytes32) {
        return blobhash(0);
    }
}
