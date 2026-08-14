// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Escape sequences are decoded before the label is compared, so this one
    // does name `evmasm`.
    function hexEscapedDialect() public pure {
        assembly "\x65vmasm" {}
    }
}
