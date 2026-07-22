// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() external {}
    function f(uint x) external {}
}

contract B {
    function g() public {
        // Neither overload is callable via the contract type name, but each
        // reference still disambiguates to the overload matching the arguments.
        A.f();
        A.f(1);
    }
}
