// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() public {}
    function f(uint x) external {}
}

contract B is A {
    function g() public {
        // A public overload selected via the base contract type name is a
        // valid qualified call from a derived contract.
        A.f();
    }
}
