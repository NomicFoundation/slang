// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() external pure virtual {}

    function g() internal pure virtual {}
}

contract B is A {
    function missing() internal pure {
        // `super` only reaches internally visible members.
        super.f();
    }

    function present() internal pure {
        super.g();
    }
}
