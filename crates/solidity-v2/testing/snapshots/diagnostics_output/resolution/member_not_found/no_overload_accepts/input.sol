// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f(uint8) external {}

    function f(bool) external {}
}

contract C {
    function no_overload_accepts(A a) internal {
        // The member exists, but a string converts to neither parameter.
        a.f("nope");
    }

    function one_overload_accepts(A a) internal {
        a.f(true);
    }
}
