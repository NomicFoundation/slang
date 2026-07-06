// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    struct S {
        uint x;
    }
}

contract C is I {
    struct S {
        uint y;
    }
}
