// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a `return` in a `try` clause, which opens a scope of its own, still
// returns from the modifier.

contract C {
    function g() external pure returns (uint256) {
        return 1;
    }

    modifier m() {
        try this.g() returns (uint256 v) {
            return v;
        } catch {
            return 2;
        }
        _;
    }
}
