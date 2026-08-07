// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Control: escape sequences are decoded before the label is compared, so
    // this one does name `evmasm`.
    function hexEscapedDialect() public pure {
        assembly "\x65vmasm" {}
    }

    // Control: the same label, spelled with a unicode escape.
    function unicodeEscapedDialect() public pure {
        assembly "\u0065vmasm" {}
    }

    // Control: a line continuation decodes to nothing at all.
    function lineContinuationInDialect() public pure {
        assembly "evm\
asm" {}
    }

    // Control: every character encoded still decodes to `evmasm`.
    function fullyEncodedDialect() public pure {
        assembly "\x65\x76\x6d\x61\x73\x6d" {}
    }

    // The decoded value is what gets checked, and this one isn't `evmasm`.
    function encodedUnknownDialect() public pure {
        assembly "\x65vm" {}
    }
}
