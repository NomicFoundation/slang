// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Control: naming a dialect is optional.
    function noDialect() public pure {
        assembly {}
    }

    // Control: the only supported dialect.
    function doubleQuotedDialect() public pure {
        assembly "evmasm" {}
    }

    // Control: the quoting style of the label doesn't matter.
    function singleQuotedDialect() public pure {
        assembly 'evmasm' {}
    }
}
