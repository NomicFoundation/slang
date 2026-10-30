// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f() private {}

    function g() internal {
        // A private member is not reachable through the library name, not
        // even from within its own library.
        L.f();
    }
}
