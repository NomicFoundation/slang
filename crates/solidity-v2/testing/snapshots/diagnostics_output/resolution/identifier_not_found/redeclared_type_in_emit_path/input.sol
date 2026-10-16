// SPDX-License-Identifier: MIT
pragma solidity *;

// Only the redeclaration is reported, not the member of the redeclared type.
contract A {
    event E();
}
contract A {}

contract D {
    function f() public {
        emit A.E();
    }
}
