// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a `return` with a value belongs to a function, including one that
// invokes a modifier.

contract C {
    modifier m() {
        _;
    }

    function f() public pure m returns (uint256) {
        return 1;
    }
}
