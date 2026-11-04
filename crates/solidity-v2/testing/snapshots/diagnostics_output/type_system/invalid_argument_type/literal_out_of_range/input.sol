// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint8) internal {}

    function g() public {
        f(300);
    }
}
