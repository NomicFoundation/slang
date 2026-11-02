// SPDX-License-Identifier: MIT
pragma solidity *;

function f() {}

library L {
    function f() external pure {}

    function g() public {
        // The external `L.f` is not visible by name, so this calls the free `f`.
        f();
    }
}
