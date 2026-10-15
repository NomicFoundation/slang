// SPDX-License-Identifier: MIT
pragma solidity *;

// `Big` spans `2**256` slots, one past the end of storage.
struct Big {
    uint256[2 ** 255] a;
    uint256[2 ** 255] b;
}

contract C {
    Big s;
}
