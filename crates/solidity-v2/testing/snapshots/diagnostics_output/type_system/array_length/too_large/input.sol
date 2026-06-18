// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A length larger than 2**256 - 1 is not allowed.
    uint256[2 ** 256] x;
}
