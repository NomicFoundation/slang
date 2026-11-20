// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(bytes calldata) public {}
}

contract C {
    function g() public {
        L.f("x");
    }
}
