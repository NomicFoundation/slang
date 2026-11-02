// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g(uint256 a) public pure {}

    function f() public pure {
        g({a: x});
    }
}
