// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // slang currently rejects array lengths above 2**64 - 1.
    uint256[2 ** 64] x;
}
