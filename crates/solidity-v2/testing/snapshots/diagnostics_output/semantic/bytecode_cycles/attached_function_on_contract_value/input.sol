// SPDX-License-Identifier: MIT
pragma solidity *;

// `using L for C` attaches g to values of the contract type C. The call
// runs within the caller's bytecode like any internal library call.

library L {
    function g(C self) internal {
        new B();
    }
}

contract C {
    using L for C;

    function f(C other) public {
        other.g();
    }
}

contract B {
    constructor() {
        new C();
    }
}
