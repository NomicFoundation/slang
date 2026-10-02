// SPDX-License-Identifier: MIT
pragma solidity *;

// The array itself is one slot, but its elements do not fit in storage.
struct Big {
    uint256[2 ** 255] a;
    uint256[2 ** 255] b;
}

contract C {
    Big[] xs;
}
