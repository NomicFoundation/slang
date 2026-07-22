// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() external {}
    function f(uint x) external {}
}

contract B {
    function g() external {
        // Both should be invalid
        A.f();
        A.f(1);
    }
}
