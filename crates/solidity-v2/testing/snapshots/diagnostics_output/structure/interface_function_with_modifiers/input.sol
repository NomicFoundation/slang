// SPDX-License-Identifier: MIT
pragma solidity *;

// A function declared in an interface cannot have modifier invocations.
interface I {
    function f() external m pure returns (uint);
    modifier m() {
        _;
    }
}
