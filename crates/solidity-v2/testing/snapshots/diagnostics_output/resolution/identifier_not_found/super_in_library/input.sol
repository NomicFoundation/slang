// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f() internal pure {}

    function g() internal pure {
        super.f();
    }
}
