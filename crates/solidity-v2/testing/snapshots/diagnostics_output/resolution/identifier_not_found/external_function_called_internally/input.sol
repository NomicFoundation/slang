// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() external pure {}

    function g() public pure {
        f();
    }
}
