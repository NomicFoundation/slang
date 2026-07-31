// SPDX-License-Identifier: MIT
pragma solidity *;

// Every contract on the cycle reports the reference through which it reaches
// the cycle.

contract A {
    function f() public {
        new B();
    }
}

contract B {
    function f() public {
        new C();
    }
}

contract C {
    function f() public {
        new A();
    }
}
