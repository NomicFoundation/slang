// SPDX-License-Identifier: MIT
pragma solidity *;

// An error is not a type.
error Er();

contract C {
    function f() public {
        Er y;
    }
}
