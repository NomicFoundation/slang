// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f() external {}
}

contract C {
    function g() public {
        abi.encodeCall(L.f, ());
    }
}
