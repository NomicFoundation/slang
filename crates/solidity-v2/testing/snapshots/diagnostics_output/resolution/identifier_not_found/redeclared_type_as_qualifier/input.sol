// SPDX-License-Identifier: MIT
pragma solidity *;

// Only the redeclaration is reported, not the member of the redeclared type.
contract A {
    struct S {
        bool x;
    }
}
contract A {}

contract D {
    A.S s;
}
