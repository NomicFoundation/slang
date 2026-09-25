// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() internal returns (address) {}

    function f() public pure returns (address) {
        return address(g);
    }
}
