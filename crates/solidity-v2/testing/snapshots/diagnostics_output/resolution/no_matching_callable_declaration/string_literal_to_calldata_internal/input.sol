// SPDX-License-Identifier: MIT
pragma solidity *;

// A string literal does not convert to `bytes calldata`, so no overload of an
// internal call matches it.
contract C {
    function f(bytes calldata) internal {}

    function f(uint256) internal {}

    function g() public {
        f("x");
    }
}
