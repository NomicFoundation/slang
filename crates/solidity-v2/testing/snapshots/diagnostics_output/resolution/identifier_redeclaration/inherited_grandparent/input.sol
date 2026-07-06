// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    uint a;
}

contract B is A {}

contract C is B {
    uint a;
}
