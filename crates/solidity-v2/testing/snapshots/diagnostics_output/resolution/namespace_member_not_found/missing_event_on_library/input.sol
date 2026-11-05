// SPDX-License-Identifier: MIT
pragma solidity *;

library L {}

contract C {
    function f() public {
        emit L.E();
    }
}
