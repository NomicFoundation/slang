// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A scientific literal whose value is too large to represent is rejected
    // as an invalid array length (and must not be expensive to evaluate).
    uint256[1.111111E1111111111111] a;
}
