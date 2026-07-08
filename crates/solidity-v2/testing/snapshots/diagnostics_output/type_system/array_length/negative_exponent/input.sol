// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A negative exponent folds to a non-integer (1/2) length.
    uint256[2 ** -1] x;
}
