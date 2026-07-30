// SPDX-License-Identifier: MIT
pragma solidity *;

struct S {
    uint256 a;
    uint256 b;
}

contract C {
    // Invalid: the named argument `a` is provided twice, here in a state
    // variable initializer rather than a function body, to exercise the check
    // outside of statement position.
    S s = S({a: 1, a: 2});
}
