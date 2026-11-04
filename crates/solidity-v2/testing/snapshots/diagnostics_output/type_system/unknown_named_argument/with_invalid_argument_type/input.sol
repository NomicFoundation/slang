// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 a, bool b) internal {}

    function g() public {
        f({a: true, c: 2});
    }
}
