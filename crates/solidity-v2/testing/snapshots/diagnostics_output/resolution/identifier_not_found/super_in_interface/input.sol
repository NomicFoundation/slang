// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    function f() external;

    function g() external {
        super.f();
    }
}
