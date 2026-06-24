// SPDX-License-Identifier: MIT
pragma solidity *;

// A parameter whose type is not `bytes calldata` is invalid.
contract A {
    fallback(uint256) external {}
}

// Declaring returns without the accepted signature is invalid.
contract B {
    fallback() external returns (uint256) {}
}

// A `bytes` parameter at the wrong data location is invalid.
contract C {
    fallback(bytes memory) external returns (bytes memory) {}
}

// A `bytes` return at the wrong data location is invalid.
contract D {
    fallback(bytes calldata) external returns (bytes calldata) {}
}

// More than one return value is invalid.
contract E {
    fallback() external returns (bytes memory, bytes memory) {}
}

// The two accepted forms are valid and must not be flagged:
// `fallback()` and `fallback(bytes calldata) returns (bytes memory)`.
contract F {
    fallback() external {}
}

contract G {
    fallback(bytes calldata) external returns (bytes memory) {}
}
