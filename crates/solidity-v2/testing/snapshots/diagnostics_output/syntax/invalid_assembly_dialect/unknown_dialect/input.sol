// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function unknownDialect() public pure {
        // `evmasm` is the only dialect an assembly statement can name.
        assembly "evm" {}
    }

    function dialectIsCaseSensitive() public pure {
        assembly "EVMASM" {}
    }

    function emptyDialect() public pure {
        assembly "" {}
    }

    function whitespaceIsPartOfTheDialect() public pure {
        assembly "evmasm " {}
    }
}
