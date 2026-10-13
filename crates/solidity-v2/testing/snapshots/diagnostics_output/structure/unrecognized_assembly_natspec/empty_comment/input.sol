// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        // solc ignores empty comments, but slang only allows the memory-safe marker
        /** */
        assembly {}

        ///
        ///
        assembly {}
    }
}
