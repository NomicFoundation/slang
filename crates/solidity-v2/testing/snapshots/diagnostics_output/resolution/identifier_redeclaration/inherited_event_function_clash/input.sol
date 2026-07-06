// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function dup() public returns (uint) {
        return 1;
    }
}

contract B {
    event dup();
}

contract C is A, B {}
