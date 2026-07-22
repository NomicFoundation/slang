// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f(uint x) external {}
    function f(bool b) external {}
}

contract B {
    function g() public {
        // Both overloads are selected by their named arguments, and neither
        // is callable via the contract type name.
        A.f({x: 1});
        A.f({b: true});
    }
}
