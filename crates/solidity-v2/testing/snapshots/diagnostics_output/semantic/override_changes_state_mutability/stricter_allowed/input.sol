// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function foo() internal view virtual returns (uint256) {}

    function bar() external pure virtual returns (uint256);
}

// Stricter mutability is always fine.
abstract contract B is A {
    function foo() internal pure virtual override returns (uint256) {}
}

abstract contract C is A {
    function foo() internal view virtual override returns (uint256) {}
}

abstract contract D is B, C {
    function foo() internal pure override(B, C) returns (uint256) {}
}

contract E is A {
    // A `constant` variable's getter is `pure`, matching the overridden one.
    uint256 public constant override bar = 7;
}
