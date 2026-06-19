// SPDX-License-Identifier: MIT
pragma solidity *;

// Companion to `overloaded_internal_selected`: the same mixed overload set, but
// here the call selects the *external* overload through the contract type name,
// which remains invalid (error 3419).

contract A {
    function f() internal {}
    function f(uint x) external {}
}

contract B is A {
    function g() public {
        // TypeError 3419: Cannot call function via contract type name.
        A.f(1);
    }
}
