// SPDX-License-Identifier: MIT
pragma solidity *;

// Internal library functions execute within the caller's bytecode, so the
// creation inside `L.f` belongs to D.

library L {
    function f() internal {
        new C();
    }
}

contract D {
    function f() public {
        L.f();
    }
}

contract C {
    constructor() {
        new D();
    }
}
