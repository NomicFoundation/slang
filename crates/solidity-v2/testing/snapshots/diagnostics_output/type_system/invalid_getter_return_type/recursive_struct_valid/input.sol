// SPDX-License-Identifier: MIT
pragma solidity *;

// The recursive member is an array, which the getter omits.
contract C {
    struct S {
        S[] children;
        uint256 a;
    }

    S public s;
}
