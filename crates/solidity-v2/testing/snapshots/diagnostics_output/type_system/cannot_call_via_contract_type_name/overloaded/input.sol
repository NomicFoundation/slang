// SPDX-License-Identifier: MIT
pragma solidity *;

// Overloaded functions accessed through a contract type name resolve to an
// ambiguous (`Undetermined`) member, but calling them is still invalid: every
// candidate is a non-callable declaration.

contract A {
    function f() external {}
    function f(uint x) external {}
}

contract B {
    function g() external {
        // TypeError 3419: Cannot call function via contract type name.
        A.f();
        A.f(1);
    }
}
