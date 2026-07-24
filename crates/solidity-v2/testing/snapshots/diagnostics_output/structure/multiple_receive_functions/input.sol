// SPDX-License-Identifier: MIT
pragma solidity *;

// A contract may declare at most one receive function. Every receive beyond
// the first is flagged.
contract C {
    receive() external payable {}
    receive() external payable {}
    receive() external payable {}
}

// A contract with a single receive function is valid and must not be flagged.
contract D {
    receive() external payable {}
}
