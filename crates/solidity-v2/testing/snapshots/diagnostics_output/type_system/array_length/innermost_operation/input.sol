// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // The failing operation is the inner `1 / 0`; the enclosing `* 2` and the
    // length use itself must not relocate the diagnostic.
    uint256[(1 / 0) * 2] x;
}
