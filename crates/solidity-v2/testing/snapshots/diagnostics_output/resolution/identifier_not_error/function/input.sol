// SPDX-License-Identifier: MIT
pragma solidity *;

// A `revert` names a function.
contract C {
    function g() public {}

    function f() public {
        revert g();
    }
}
