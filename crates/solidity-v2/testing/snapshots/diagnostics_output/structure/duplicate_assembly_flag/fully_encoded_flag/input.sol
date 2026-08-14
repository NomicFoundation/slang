// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function repeatedFullyEncoded() public pure {
        // Every character encoded still decodes to `memory-safe`.
        assembly ("memory-safe", "\x6d\x65\x6d\x6f\x72\x79\x2d\x73\x61\x66\x65") {}
    }
}
