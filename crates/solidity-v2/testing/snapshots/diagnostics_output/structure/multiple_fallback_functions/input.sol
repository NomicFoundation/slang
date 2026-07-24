// SPDX-License-Identifier: MIT
pragma solidity *;

// A contract may declare at most one fallback function. Every fallback beyond
// the first is flagged.
contract C {
    fallback() external {}
    fallback(bytes calldata) external returns (bytes memory) {}
}

// A contract with a single fallback function is valid and must not be flagged.
contract D {
    fallback() external {}
}
