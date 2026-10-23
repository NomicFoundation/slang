// SPDX-License-Identifier: MIT
pragma solidity *;

// Only the redeclaration is reported, not the use of the redeclared type.
contract A {}
contract A {}

contract D {
    A a;
}
