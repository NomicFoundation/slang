// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    struct S {
        uint x;
    }
}

contract B is A {
    struct S {
        uint y;
    }
}
