// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    uint internal constant X = 1;
}

contract B is A {
    uint constant X = 2;
}
