// SPDX-License-Identifier: MIT
pragma solidity *;

// A `pure` fallback is invalid: a fallback must be payable or non-payable.
contract A {
    fallback() external pure {}
}

// A `view` fallback is invalid for the same reason.
contract B {
    fallback() external view {}
}

// Non-payable and payable fallbacks are valid and must not be flagged.
contract C {
    fallback() external {}
}

contract D {
    fallback() external payable {}
}
