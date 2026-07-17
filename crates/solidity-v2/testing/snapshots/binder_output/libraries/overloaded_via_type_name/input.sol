// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f() internal {}
    function f(uint x) internal {}
}

contract C {
    function g() public {
        // Library members via the type name stay normal callables, and each
        // reference disambiguates to the overload matching the arguments.
        L.f();
        L.f(1);
    }
}
