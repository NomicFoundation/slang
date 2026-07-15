// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    // A function declared in a library must have an implementation body.
    function f1() internal returns (uint);

    // A library function with an implementation body is allowed.
    function f2() internal returns (uint) {}
}

interface I {
    // A function without a body inside an interface is allowed.
    function f3() external returns (uint);
}
