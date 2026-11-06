// SPDX-License-Identifier: MIT
pragma solidity *;

// A function is not a type.
contract C {
    function g() public {}

    function f() public {
        g y;
    }
}
