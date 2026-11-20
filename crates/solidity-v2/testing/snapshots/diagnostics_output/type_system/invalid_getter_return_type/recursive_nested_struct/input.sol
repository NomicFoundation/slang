// SPDX-License-Identifier: MIT
pragma solidity *;

// A nested struct is returned whole, so it cannot be recursive.
contract C {
    struct T {
        T[] children;
    }

    struct S {
        T t;
        uint256 a;
    }

    S public s;
}
