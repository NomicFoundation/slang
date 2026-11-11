// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes calldata) internal {}

    function g() public {
        f("x");
    }
}
