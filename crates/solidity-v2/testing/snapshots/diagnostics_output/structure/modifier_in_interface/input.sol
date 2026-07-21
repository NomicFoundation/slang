// SPDX-License-Identifier: MIT
pragma solidity *;

// An interface cannot define or declare a modifier.
interface I {
    modifier m() {
        _;
    }
}

// A contract may define a modifier, and must not be flagged.
contract C {
    modifier m() {
        _;
    }
}
