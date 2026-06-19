// SPDX-License-Identifier: MIT
pragma solidity *;

// An overload set mixing an internal and an external function. Selecting the
// internal overload through the base contract type name (`A.f()`) is a valid
// qualified base call, even though the sibling `f(uint)` is external. Only the
// *selected* overload's visibility decides whether error 3419 applies, so this
// file must compile cleanly.

contract A {
    function f() internal {}
    function f(uint x) external {}
}

contract B is A {
    function g() public {
        A.f();
    }
}
