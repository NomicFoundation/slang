// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() external {}
}

contract C {
    function missing(A a) internal {
        a.nope();
    }

    function present(A a) internal {
        a.f();
    }
}
