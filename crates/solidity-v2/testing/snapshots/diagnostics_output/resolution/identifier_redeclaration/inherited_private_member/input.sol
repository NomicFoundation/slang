// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    uint private a;
    uint private constant X = 1;
}

contract B is A {
    uint a;
    uint constant X = 2;
}
