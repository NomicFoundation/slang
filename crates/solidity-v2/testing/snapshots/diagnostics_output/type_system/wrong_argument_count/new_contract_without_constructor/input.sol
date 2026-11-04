// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {}

contract D {
    function g() public {
        new C(1);
    }
}
