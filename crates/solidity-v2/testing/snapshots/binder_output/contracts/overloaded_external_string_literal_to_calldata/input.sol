// SPDX-License-Identifier: MIT
pragma solidity *;

// An external call decodes a `bytes calldata` parameter into memory, so a
// string literal matches it.
contract C {
    function f(bytes calldata) external {}

    function f(uint256) external {}

    function g() public {
        this.f("x");
    }
}
