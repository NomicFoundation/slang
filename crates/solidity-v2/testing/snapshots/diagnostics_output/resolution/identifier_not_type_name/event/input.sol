// SPDX-License-Identifier: MIT
pragma solidity *;

// An event is not a type.
contract C {
    event E();

    function f() public {
        E y;
    }
}
