// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256, uint256 b) internal {}

    function g() public {
        f({b: 1, c: 2});
    }
}
