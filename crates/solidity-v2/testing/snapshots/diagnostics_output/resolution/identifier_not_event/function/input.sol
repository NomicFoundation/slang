// SPDX-License-Identifier: MIT
pragma solidity *;

// An `emit` names a function.
contract C {
    function g() public {}

    function f() public {
        emit g();
    }
}
