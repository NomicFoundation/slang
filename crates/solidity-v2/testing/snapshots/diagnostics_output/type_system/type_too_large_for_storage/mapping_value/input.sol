// SPDX-License-Identifier: MIT
pragma solidity *;

// The mapping itself is one slot, but its values do not fit in storage.
struct Big {
    uint256[2 ** 255] a;
    uint256[2 ** 255] b;
}

contract C {
    mapping(uint256 => Big) m;
}
