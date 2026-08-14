// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // The decoded value is what gets checked, and this one isn't `evmasm`.
    function encodedUnknownDialect() public pure {
        assembly "\x65vm" {}
    }
}
