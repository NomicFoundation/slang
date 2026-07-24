// SPDX-License-Identifier: MIT
pragma solidity *;

// External calls run in the callee's already deployed bytecode, so no
// bytecode is embedded and there is no cycle.

contract A {
    B b;

    function f() public {
        b.g();
    }
}

contract B {
    function g() public {
        new A();
    }
}
