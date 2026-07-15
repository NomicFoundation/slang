// SPDX-License-Identifier: MIT
pragma solidity *;

// A struct that holds a recursive struct by value is itself infinitely sized,
// so it is reported even though it is not a member of the cycle. solc reports
// only Inner, as its nested visitation checks Inner first and stops there.

contract Test {
    struct Outer {
        Inner x;
    }
    struct Inner {
        Inner y;
    }
}
