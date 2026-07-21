// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    struct S {
        uint x;
    }
}

interface I1 is I {
    struct S {
        uint y;
    }
}
