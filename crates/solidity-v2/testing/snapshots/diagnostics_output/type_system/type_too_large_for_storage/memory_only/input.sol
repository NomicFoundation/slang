// SPDX-License-Identifier: MIT
pragma solidity *;

// `Big` is never stored, so its size does not matter.
struct Big {
    uint256[2 ** 255] a;
    uint256[2 ** 255] b;
}

contract C {
    function f() public pure {
        Big[] memory xs;
        xs;
    }
}
