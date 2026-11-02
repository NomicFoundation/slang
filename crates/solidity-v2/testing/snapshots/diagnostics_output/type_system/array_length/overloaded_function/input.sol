// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public {}
    function f(uint256) public {}
    // An overloaded function name resolves ambiguously and is not a
    // compile-time constant.
    uint256[f] a;
}
