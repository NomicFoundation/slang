// SPDX-License-Identifier: MIT
pragma solidity *;

contract D {
    function g() internal {}
}

contract C {
    function f() public {
        D.g();
    }
}
