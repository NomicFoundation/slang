// SPDX-License-Identifier: MIT
pragma solidity *;

// D inherits foo from C, so both depend on A through the same `new A`
// expression. Each referencing expression is reported only once.

contract A {
    function foo() public {
        new D();
    }
}

contract C {
    function foo() public {
        new A();
    }
}

contract D is C {}
