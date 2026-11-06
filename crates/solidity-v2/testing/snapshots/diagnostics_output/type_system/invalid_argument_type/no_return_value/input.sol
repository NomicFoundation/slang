// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256) internal {}
    function h() internal {}

    function g() public {
        f(h());
    }
}
