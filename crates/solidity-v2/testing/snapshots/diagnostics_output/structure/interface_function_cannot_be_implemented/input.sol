// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    // A function declared in an interface cannot have an implementation body.
    function f1() external {}

    // A function declared in an interface without a body is allowed.
    function f2() external;
}

contract C {
    // A function with a body inside a contract is allowed.
    function f3() external {}
}
