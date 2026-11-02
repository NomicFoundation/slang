// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    event E();
}

contract C {
    function f() public {
        emit A.Nope();
    }
}
