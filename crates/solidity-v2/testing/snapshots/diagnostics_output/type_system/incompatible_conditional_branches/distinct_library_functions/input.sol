// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f() external {}
    function g() external {}
}

contract C {
    // Both branches are delegate calls to the library, so they share a type
    // even though they name different functions.
    function h(bool b) public {
        (b ? L.f : L.g)();
    }
}
