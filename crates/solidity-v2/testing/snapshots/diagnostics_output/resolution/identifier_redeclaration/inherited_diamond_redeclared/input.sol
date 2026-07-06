// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    uint a;
}

contract B {
    uint a;
}

contract D is A, B {
    uint a;
}
