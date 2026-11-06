// SPDX-License-Identifier: MIT
pragma solidity *;

// An `emit` names an error rather than an event.
error Er();

contract C {
    function f() public {
        emit Er();
    }
}
