// SPDX-License-Identifier: MIT
pragma solidity *;

// Only the redeclaration is reported, not the use of the redeclared type.
contract C {
    struct S {
        bool x;
    }
    struct S {
        bool x;
    }
}

function foo() pure {
    C.S memory s;
}
