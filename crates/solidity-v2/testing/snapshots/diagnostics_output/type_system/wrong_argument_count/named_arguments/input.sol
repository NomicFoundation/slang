// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 a, uint256 b) internal {}

    function g() public {
        f({a: 1});
    }
}
