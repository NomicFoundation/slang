// SPDX-License-Identifier: MIT
pragma solidity *;

// `W` fits, but the elements of its `xs` member do not.
struct Big {
    uint256[2 ** 255] a;
    uint256[2 ** 255] b;
}

struct W {
    Big[] xs;
}

contract C {
    W w;
}
