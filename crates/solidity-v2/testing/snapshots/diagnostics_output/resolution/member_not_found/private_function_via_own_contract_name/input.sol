// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() private {}

    function g() public {
        // A private member is not reachable through the contract name, not
        // even from within its own contract.
        C.f();
    }
}
