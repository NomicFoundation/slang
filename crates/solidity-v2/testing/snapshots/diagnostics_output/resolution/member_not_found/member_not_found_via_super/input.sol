// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() internal pure virtual {}
}

contract B is A {
    function missing() internal pure {
        super.nope();
    }

    function present() internal pure {
        super.f();
    }
}
