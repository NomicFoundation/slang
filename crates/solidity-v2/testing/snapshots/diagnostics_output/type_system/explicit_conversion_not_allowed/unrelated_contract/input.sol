// SPDX-License-Identifier: MIT
pragma solidity *;

contract D {}

contract C {
    function f(D d) public pure returns (C, C) {
        return (C(d), C(address(d)));
    }
}
