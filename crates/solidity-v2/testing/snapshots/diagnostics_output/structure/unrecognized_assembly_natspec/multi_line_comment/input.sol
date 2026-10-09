// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        // solc marks these blocks as memory safe, but slang only recognizes a single `///` line
        /** @solidity memory-safe-assembly */
        assembly {}

        /**
         * @solidity memory-safe-assembly
         */
        assembly {}

        /// @solidity memory-safe-assembly
        ///
        assembly {}
    }
}
