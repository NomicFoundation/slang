// SPDX-License-Identifier: MIT
pragma solidity *;

// A `revert` names an event rather than an error.
event E();

contract C {
    function f() public {
        revert E();
    }
}
