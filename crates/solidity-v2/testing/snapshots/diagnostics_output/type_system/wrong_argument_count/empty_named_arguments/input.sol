// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 a) internal {}

    function g() public {
        f({});
    }
}
