// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // The same label, spelled with a unicode escape.
    function unicodeEscapedDialect() public pure {
        assembly "\u0065vmasm" {}
    }
}
