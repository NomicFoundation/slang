// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Whitespace is part of the dialect label, and isn't trimmed away.
    function whitespaceIsPartOfTheDialect() public pure {
        assembly "evmasm " {}
    }
}
