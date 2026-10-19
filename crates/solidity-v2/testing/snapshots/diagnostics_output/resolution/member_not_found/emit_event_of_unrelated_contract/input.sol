// SPDX-License-Identifier: MIT
pragma solidity *;

contract D {
    event E();
}

contract C {
    function f() public {
        emit D.E();
    }
}
