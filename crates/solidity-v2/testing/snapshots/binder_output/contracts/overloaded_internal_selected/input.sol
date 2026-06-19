// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() internal {}
    function f(uint x) external {}
}

contract B is A {
    function g() public {
        A.f();
        A.f(1);
    }
}
