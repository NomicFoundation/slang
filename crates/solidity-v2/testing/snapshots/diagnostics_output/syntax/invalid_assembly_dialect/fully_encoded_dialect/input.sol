// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Every character encoded still decodes to `evmasm`.
    function fullyEncodedDialect() public pure {
        assembly "\x65\x76\x6d\x61\x73\x6d" {}
    }
}
