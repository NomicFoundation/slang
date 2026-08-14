// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A line continuation decodes to nothing at all.
    function lineContinuationInDialect() public pure {
        assembly "evm\
asm" {}
    }
}
