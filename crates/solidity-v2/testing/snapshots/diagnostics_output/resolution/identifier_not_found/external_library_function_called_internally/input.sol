// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f() external pure {}

    function g() public pure {
        f();
    }
}
