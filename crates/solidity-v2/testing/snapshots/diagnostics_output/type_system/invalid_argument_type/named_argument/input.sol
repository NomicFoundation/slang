// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 a, bool b) internal {}

    function g() public {
        f({b: 1, a: 2});
    }
}
