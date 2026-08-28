// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f() internal pure {}
}

contract C {
    function missing() internal pure {
        L.nope();
    }

    function present() internal pure {
        L.f();
    }
}
