// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        // Neither marks the block as memory safe
        /// @notice Writes to scratch space
        assembly {}

        /// Writes to scratch space
        assembly {}
    }
}
