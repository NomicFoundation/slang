// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256 constant N = 3;

    // A literal and constant expressions (addition, division, exponentiation)
    // that fold to a valid integer length are all accepted.
    uint256[5] x;
    uint256[N + 1] y;
    uint256[100 / 4] z;
    uint256[2 ** 4] w;
}
