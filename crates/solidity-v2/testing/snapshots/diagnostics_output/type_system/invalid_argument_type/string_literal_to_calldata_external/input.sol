// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes calldata) external {}

    function g() public {
        this.f("x");
    }
}
