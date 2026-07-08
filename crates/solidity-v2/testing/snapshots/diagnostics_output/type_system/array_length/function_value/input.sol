// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public {}
    // A function reference is not a compile-time constant.
    uint256[f] a;
}
