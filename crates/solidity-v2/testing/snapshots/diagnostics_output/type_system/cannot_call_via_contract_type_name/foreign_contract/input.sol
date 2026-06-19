// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() external {}
    function g() external pure {}
    function h() public pure {}
}

contract B {
    function i() external {
        // Cannot call function via contract type name.
        A.f();
        A.g();
        A.h(); 
    }
}
