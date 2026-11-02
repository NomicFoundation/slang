// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256 x;

    modifier m() {
        _;
    }

    function f() public x.m {}
}
