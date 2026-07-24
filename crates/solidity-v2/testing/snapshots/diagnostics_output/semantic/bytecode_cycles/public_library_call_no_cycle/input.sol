// SPDX-License-Identifier: MIT
pragma solidity *;

// Calling a public library function is a delegatecall into the deployed
// library, so C does not embed L's bytecode.

library L {
    function g() public {
        new C();
    }
}

contract C {
    function f() public {
        L.g();
    }
}
