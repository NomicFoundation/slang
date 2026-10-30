// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {
    function f() private {}
}

contract D is B {
    function g() public {
        // A private member is not reachable through the contract name.
        B.f();
    }
}
