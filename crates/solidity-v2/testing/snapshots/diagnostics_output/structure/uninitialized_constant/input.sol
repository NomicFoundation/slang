// SPDX-License-Identifier: MIT
pragma solidity *;

// A `constant` must be initialized with a value at its declaration site. The
// parser accepts a missing initializer, so this is reported as a structural
// diagnostic. Non-public constants are lowered to `ConstantDefinition`, while
// public ones keep the `StateVariableDefinition` shape; both paths are covered.

contract C {
    uint256 constant A;
    uint256 public constant B;
    uint256 internal constant D;

    uint256 constant OK = 1;
}
