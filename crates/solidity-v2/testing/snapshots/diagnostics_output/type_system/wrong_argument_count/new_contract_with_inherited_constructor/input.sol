// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {
    constructor(uint256 a) {}
}

contract C is B(1) {}

contract D {
    function g() public {
        new C(1);
    }
}
